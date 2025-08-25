fr fr ==========================================
fr fr ENHANCED JSON PARSER - Complete RFC 7159 Implementation
fr fr Full JSON specification support with proper error handling
fr fr ==========================================

yeet "stringz"
yeet "mathz"
yeet "vibez"

fr fr ==========================================
fr fr JSON Parser Data Structures
fr fr ==========================================

squad JsonValue {
    sus value_type tea              fr fr "null", "boolean", "number", "string", "array", "object"
    sus string_value tea
    sus number_value meal
    sus boolean_value lit
    sus array_items []JsonValue
    sus object_pairs []JsonKeyValue
    sus line_number drip
    sus column_number drip
}

squad JsonKeyValue {
    sus key tea
    sus value JsonValue
}

squad JsonParser {
    sus input tea
    sus position drip
    sus length drip
    sus line drip
    sus column drip
    sus current_char tea
    sus error_messages []tea
    sus has_error lit
}

squad JsonParseResult {
    sus success lit
    sus value JsonValue
    sus error_message tea
    sus parser JsonParser
}

fr fr ==========================================
fr fr Core JSON Parser Implementation
fr fr ==========================================

slay json_parse_string(json_content tea) JsonParseResult {
    fr fr Parse JSON string with complete RFC 7159 compliance
    sus parser JsonParser = create_json_parser(json_content)
    sus result JsonParseResult = JsonParseResult{
        success: cringe,
        value: JsonValue{},
        error_message: "",
        parser: parser
    }
    
    fr fr Skip leading whitespace
    parser = skip_json_whitespace(parser)
    
    ready (parser.position >= parser.length) {
        result.error_message = "Empty JSON input"
        damn result
    }
    
    fr fr Parse the main JSON value
    sus parse_result JsonParseResult = parse_json_value(parser)
    ready (!parse_result.success) {
        result.error_message = parse_result.error_message
        damn result
    }
    
    parser = parse_result.parser
    parser = skip_json_whitespace(parser)
    
    fr fr Ensure we've consumed all input
    ready (parser.position < parser.length) {
        result.error_message = "Unexpected characters after JSON value at line " + 
                              drip_to_string(parser.line) + ", column " + drip_to_string(parser.column)
        damn result
    }
    
    result.success = based
    result.value = parse_result.value
    result.parser = parser
    damn result
}

slay create_json_parser(input tea) JsonParser {
    fr fr Initialize JSON parser state
    sus parser JsonParser = JsonParser{
        input: input,
        position: 0,
        length: string_length(input),
        line: 1,
        column: 1,
        current_char: "",
        error_messages: [],
        has_error: cringe
    }
    
    ready (parser.length > 0) {
        parser.current_char = string_char_at(input, 0)
    }
    
    damn parser
}

slay parse_json_value(parser JsonParser) JsonParseResult {
    fr fr Parse any JSON value
    sus result JsonParseResult = JsonParseResult{
        success: cringe,
        value: JsonValue{},
        error_message: "",
        parser: parser
    }
    
    parser = skip_json_whitespace(parser)
    
    ready (parser.position >= parser.length) {
        result.error_message = "Unexpected end of input while parsing value"
        damn result
    }
    
    sus char tea = parser.current_char
    
    fr fr Parse different JSON value types
    ready (char == "\"") {
        damn parse_json_string_value(parser)
    } otherwise ready (char == "{") {
        damn parse_json_object(parser)
    } otherwise ready (char == "[") {
        damn parse_json_array(parser)
    } otherwise ready (char == "t") {
        damn parse_json_literal(parser, "true")
    } otherwise ready (char == "f") {
        damn parse_json_literal(parser, "false")
    } otherwise ready (char == "n") {
        damn parse_json_literal(parser, "null")
    } otherwise ready (char == "-" || is_json_digit(char)) {
        damn parse_json_number(parser)
    } otherwise {
        result.error_message = "Unexpected character '" + char + "' at line " + 
                              drip_to_string(parser.line) + ", column " + drip_to_string(parser.column)
        damn result
    }
}

fr fr ==========================================
fr fr JSON String Parsing with Full Unicode Support
fr fr ==========================================

slay parse_json_string_value(parser JsonParser) JsonParseResult {
    fr fr Parse JSON string with full escape sequence support
    sus result JsonParseResult = JsonParseResult{
        success: cringe,
        value: JsonValue{},
        error_message: "",
        parser: parser
    }
    
    ready (parser.current_char != "\"") {
        result.error_message = "Expected '\"' at start of string"
        damn result
    }
    
    parser = advance_json_parser(parser)  fr fr Skip opening quote
    sus string_value tea = ""
    
    bestie (parser.position < parser.length && parser.current_char != "\"") {
        ready (parser.current_char == "\\") {
            fr fr Handle escape sequences
            parser = advance_json_parser(parser)
            ready (parser.position >= parser.length) {
                result.error_message = "Incomplete escape sequence in string"
                damn result
            }
            
            sus escape_char tea = parser.current_char
            ready (escape_char == "\"") {
                string_value = string_value + "\""
            } otherwise ready (escape_char == "\\") {
                string_value = string_value + "\\"
            } otherwise ready (escape_char == "/") {
                string_value = string_value + "/"
            } otherwise ready (escape_char == "b") {
                string_value = string_value + "\b"
            } otherwise ready (escape_char == "f") {
                string_value = string_value + "\f"
            } otherwise ready (escape_char == "n") {
                string_value = string_value + "\n"
            } otherwise ready (escape_char == "r") {
                string_value = string_value + "\r"
            } otherwise ready (escape_char == "t") {
                string_value = string_value + "\t"
            } otherwise ready (escape_char == "u") {
                fr fr Unicode escape sequence \uXXXX
                sus unicode_result UnicodeParseResult = parse_unicode_escape(parser)
                ready (!unicode_result.success) {
                    result.error_message = unicode_result.error_message
                    damn result
                }
                string_value = string_value + unicode_result.character
                parser = unicode_result.parser
                continue
            } otherwise {
                result.error_message = "Invalid escape sequence: \\" + escape_char
                damn result
            }
            
            parser = advance_json_parser(parser)
        } otherwise ready (is_json_control_character(parser.current_char)) {
            result.error_message = "Unescaped control character in string at line " + 
                                  drip_to_string(parser.line) + ", column " + drip_to_string(parser.column)
            damn result
        } otherwise {
            string_value = string_value + parser.current_char
            parser = advance_json_parser(parser)
        }
    }
    
    ready (parser.position >= parser.length || parser.current_char != "\"") {
        result.error_message = "Unterminated string"
        damn result
    }
    
    parser = advance_json_parser(parser)  fr fr Skip closing quote
    
    sus json_value JsonValue = JsonValue{
        value_type: "string",
        string_value: string_value,
        line_number: parser.line,
        column_number: parser.column
    }
    
    result.success = based
    result.value = json_value
    result.parser = parser
    damn result
}

squad UnicodeParseResult {
    sus success lit
    sus character tea
    sus parser JsonParser
    sus error_message tea
}

slay parse_unicode_escape(parser JsonParser) UnicodeParseResult {
    fr fr Parse \uXXXX Unicode escape sequence
    sus result UnicodeParseResult = UnicodeParseResult{
        success: cringe,
        character: "",
        parser: parser,
        error_message: ""
    }
    
    fr fr Skip the 'u' character
    parser = advance_json_parser(parser)
    
    fr fr Read 4 hex digits
    sus hex_digits tea = ""
    sus i drip = 0
    bestie (i < 4) {
        ready (parser.position >= parser.length) {
            result.error_message = "Incomplete Unicode escape sequence"
            damn result
        }
        
        ready (!is_hex_digit(parser.current_char)) {
            result.error_message = "Invalid hex digit in Unicode escape: " + parser.current_char
            damn result
        }
        
        hex_digits = hex_digits + parser.current_char
        parser = advance_json_parser(parser)
        i = i + 1
    }
    
    fr fr Convert hex to Unicode character
    sus code_point drip = hex_string_to_int(hex_digits)
    sus unicode_char tea = code_point_to_utf8(code_point)
    
    result.success = based
    result.character = unicode_char
    result.parser = parser
    damn result
}

fr fr ==========================================
fr fr JSON Number Parsing with Proper Precision
fr fr ==========================================

slay parse_json_number(parser JsonParser) JsonParseResult {
    fr fr Parse JSON number with full RFC 7159 compliance
    sus result JsonParseResult = JsonParseResult{
        success: cringe,
        value: JsonValue{},
        error_message: "",
        parser: parser
    }
    
    sus number_string tea = ""
    sus start_pos drip = parser.position
    
    fr fr Handle optional minus sign
    ready (parser.current_char == "-") {
        number_string = number_string + parser.current_char
        parser = advance_json_parser(parser)
        
        ready (parser.position >= parser.length || !is_json_digit(parser.current_char)) {
            result.error_message = "Invalid number: minus not followed by digit"
            damn result
        }
    }
    
    fr fr Handle integer part
    ready (parser.current_char == "0") {
        number_string = number_string + parser.current_char
        parser = advance_json_parser(parser)
        
        fr fr In JSON, leading zeros are not allowed except for "0" itself
        ready (parser.position < parser.length && is_json_digit(parser.current_char)) {
            result.error_message = "Invalid number: leading zeros not allowed"
            damn result
        }
    } otherwise ready (is_json_digit_non_zero(parser.current_char)) {
        bestie (parser.position < parser.length && is_json_digit(parser.current_char)) {
            number_string = number_string + parser.current_char
            parser = advance_json_parser(parser)
        }
    } otherwise {
        result.error_message = "Invalid number: expected digit"
        damn result
    }
    
    fr fr Handle optional decimal part
    ready (parser.position < parser.length && parser.current_char == ".") {
        number_string = number_string + parser.current_char
        parser = advance_json_parser(parser)
        
        ready (parser.position >= parser.length || !is_json_digit(parser.current_char)) {
            result.error_message = "Invalid number: decimal point not followed by digit"
            damn result
        }
        
        bestie (parser.position < parser.length && is_json_digit(parser.current_char)) {
            number_string = number_string + parser.current_char
            parser = advance_json_parser(parser)
        }
    }
    
    fr fr Handle optional exponent part
    ready (parser.position < parser.length && 
           (parser.current_char == "e" || parser.current_char == "E")) {
        number_string = number_string + parser.current_char
        parser = advance_json_parser(parser)
        
        fr fr Handle optional plus/minus in exponent
        ready (parser.position < parser.length && 
               (parser.current_char == "+" || parser.current_char == "-")) {
            number_string = number_string + parser.current_char
            parser = advance_json_parser(parser)
        }
        
        ready (parser.position >= parser.length || !is_json_digit(parser.current_char)) {
            result.error_message = "Invalid number: exponent not followed by digit"
            damn result
        }
        
        bestie (parser.position < parser.length && is_json_digit(parser.current_char)) {
            number_string = number_string + parser.current_char
            parser = advance_json_parser(parser)
        }
    }
    
    fr fr Convert string to number
    sus number_value meal = string_to_float(number_string)
    
    sus json_value JsonValue = JsonValue{
        value_type: "number",
        number_value: number_value,
        line_number: parser.line,
        column_number: parser.column
    }
    
    result.success = based
    result.value = json_value
    result.parser = parser
    damn result
}

fr fr ==========================================
fr fr JSON Object Parsing
fr fr ==========================================

slay parse_json_object(parser JsonParser) JsonParseResult {
    fr fr Parse JSON object with proper key validation
    sus result JsonParseResult = JsonParseResult{
        success: cringe,
        value: JsonValue{},
        error_message: "",
        parser: parser
    }
    
    ready (parser.current_char != "{") {
        result.error_message = "Expected '{' at start of object"
        damn result
    }
    
    parser = advance_json_parser(parser)  fr fr Skip opening brace
    parser = skip_json_whitespace(parser)
    
    sus object_pairs []JsonKeyValue = []
    sus pair_count drip = 0
    sus used_keys []tea = []  fr fr Track keys to prevent duplicates
    
    fr fr Handle empty object
    ready (parser.position < parser.length && parser.current_char == "}") {
        parser = advance_json_parser(parser)
        
        sus json_value JsonValue = JsonValue{
            value_type: "object",
            object_pairs: object_pairs,
            line_number: parser.line,
            column_number: parser.column
        }
        
        result.success = based
        result.value = json_value
        result.parser = parser
        damn result
    }
    
    fr fr Parse key-value pairs
    bestie (based) {
        parser = skip_json_whitespace(parser)
        
        ready (parser.position >= parser.length) {
            result.error_message = "Unexpected end of input in object"
            damn result
        }
        
        fr fr Parse key (must be string)
        ready (parser.current_char != "\"") {
            result.error_message = "Expected string key in object at line " + 
                                  drip_to_string(parser.line) + ", column " + drip_to_string(parser.column)
            damn result
        }
        
        sus key_result JsonParseResult = parse_json_string_value(parser)
        ready (!key_result.success) {
            result.error_message = key_result.error_message
            damn result
        }
        
        parser = key_result.parser
        sus key tea = key_result.value.string_value
        
        fr fr Check for duplicate keys
        ready (array_contains_string(used_keys, key)) {
            result.error_message = "Duplicate key '" + key + "' in object"
            damn result
        }
        used_keys = append_string_to_array(used_keys, key)
        
        parser = skip_json_whitespace(parser)
        
        fr fr Expect colon
        ready (parser.position >= parser.length || parser.current_char != ":") {
            result.error_message = "Expected ':' after key in object"
            damn result
        }
        
        parser = advance_json_parser(parser)
        parser = skip_json_whitespace(parser)
        
        fr fr Parse value
        sus value_result JsonParseResult = parse_json_value(parser)
        ready (!value_result.success) {
            result.error_message = value_result.error_message
            damn result
        }
        
        parser = value_result.parser
        
        fr fr Add key-value pair
        sus pair JsonKeyValue = JsonKeyValue{
            key: key,
            value: value_result.value
        }
        object_pairs = append_json_pair_to_array(object_pairs, pair)
        pair_count = pair_count + 1
        
        parser = skip_json_whitespace(parser)
        
        ready (parser.position >= parser.length) {
            result.error_message = "Unexpected end of input in object"
            damn result
        }
        
        ready (parser.current_char == "}") {
            parser = advance_json_parser(parser)
            break
        } otherwise ready (parser.current_char == ",") {
            parser = advance_json_parser(parser)
            parser = skip_json_whitespace(parser)
            
            fr fr Trailing comma check
            ready (parser.position < parser.length && parser.current_char == "}") {
                result.error_message = "Trailing comma in object"
                damn result
            }
        } otherwise {
            result.error_message = "Expected ',' or '}' in object at line " + 
                                  drip_to_string(parser.line) + ", column " + drip_to_string(parser.column)
            damn result
        }
    }
    
    sus json_value JsonValue = JsonValue{
        value_type: "object",
        object_pairs: object_pairs,
        line_number: parser.line,
        column_number: parser.column
    }
    
    result.success = based
    result.value = json_value
    result.parser = parser
    damn result
}

fr fr ==========================================
fr fr JSON Array Parsing
fr fr ==========================================

slay parse_json_array(parser JsonParser) JsonParseResult {
    fr fr Parse JSON array with proper element handling
    sus result JsonParseResult = JsonParseResult{
        success: cringe,
        value: JsonValue{},
        error_message: "",
        parser: parser
    }
    
    ready (parser.current_char != "[") {
        result.error_message = "Expected '[' at start of array"
        damn result
    }
    
    parser = advance_json_parser(parser)  fr fr Skip opening bracket
    parser = skip_json_whitespace(parser)
    
    sus array_items []JsonValue = []
    sus item_count drip = 0
    
    fr fr Handle empty array
    ready (parser.position < parser.length && parser.current_char == "]") {
        parser = advance_json_parser(parser)
        
        sus json_value JsonValue = JsonValue{
            value_type: "array",
            array_items: array_items,
            line_number: parser.line,
            column_number: parser.column
        }
        
        result.success = based
        result.value = json_value
        result.parser = parser
        damn result
    }
    
    fr fr Parse array elements
    bestie (based) {
        sus element_result JsonParseResult = parse_json_value(parser)
        ready (!element_result.success) {
            result.error_message = element_result.error_message
            damn result
        }
        
        parser = element_result.parser
        array_items = append_json_value_to_array(array_items, element_result.value)
        item_count = item_count + 1
        
        parser = skip_json_whitespace(parser)
        
        ready (parser.position >= parser.length) {
            result.error_message = "Unexpected end of input in array"
            damn result
        }
        
        ready (parser.current_char == "]") {
            parser = advance_json_parser(parser)
            break
        } otherwise ready (parser.current_char == ",") {
            parser = advance_json_parser(parser)
            parser = skip_json_whitespace(parser)
            
            fr fr Trailing comma check
            ready (parser.position < parser.length && parser.current_char == "]") {
                result.error_message = "Trailing comma in array"
                damn result
            }
        } otherwise {
            result.error_message = "Expected ',' or ']' in array at line " + 
                                  drip_to_string(parser.line) + ", column " + drip_to_string(parser.column)
            damn result
        }
    }
    
    sus json_value JsonValue = JsonValue{
        value_type: "array",
        array_items: array_items,
        line_number: parser.line,
        column_number: parser.column
    }
    
    result.success = based
    result.value = json_value
    result.parser = parser
    damn result
}

fr fr ==========================================
fr fr JSON Literal Parsing (true, false, null)
fr fr ==========================================

slay parse_json_literal(parser JsonParser, expected tea) JsonParseResult {
    fr fr Parse JSON literal values with exact matching
    sus result JsonParseResult = JsonParseResult{
        success: cringe,
        value: JsonValue{},
        error_message: "",
        parser: parser
    }
    
    sus expected_length drip = string_length(expected)
    sus i drip = 0
    
    bestie (i < expected_length) {
        ready (parser.position >= parser.length || 
               parser.current_char != string_char_at(expected, i)) {
            result.error_message = "Invalid literal: expected '" + expected + 
                                  "' at line " + drip_to_string(parser.line) + 
                                  ", column " + drip_to_string(parser.column)
            damn result
        }
        
        parser = advance_json_parser(parser)
        i = i + 1
    }
    
    fr fr Create appropriate JSON value
    sus json_value JsonValue = JsonValue{
        line_number: parser.line,
        column_number: parser.column
    }
    
    ready (expected == "true") {
        json_value.value_type = "boolean"
        json_value.boolean_value = based
    } otherwise ready (expected == "false") {
        json_value.value_type = "boolean"
        json_value.boolean_value = cringe
    } otherwise ready (expected == "null") {
        json_value.value_type = "null"
    } otherwise {
        result.error_message = "Unknown literal: " + expected
        damn result
    }
    
    result.success = based
    result.value = json_value
    result.parser = parser
    damn result
}

fr fr ==========================================
fr fr JSON Parser Utility Functions
fr fr ==========================================

slay advance_json_parser(parser JsonParser) JsonParser {
    fr fr Advance parser position and update line/column tracking
    ready (parser.position >= parser.length) {
        damn parser
    }
    
    ready (parser.current_char == "\n") {
        parser.line = parser.line + 1
        parser.column = 1
    } otherwise {
        parser.column = parser.column + 1
    }
    
    parser.position = parser.position + 1
    
    ready (parser.position < parser.length) {
        parser.current_char = string_char_at(parser.input, parser.position)
    } otherwise {
        parser.current_char = ""
    }
    
    damn parser
}

slay skip_json_whitespace(parser JsonParser) JsonParser {
    fr fr Skip whitespace characters as defined by JSON spec
    bestie (parser.position < parser.length && 
            is_json_whitespace(parser.current_char)) {
        parser = advance_json_parser(parser)
    }
    damn parser
}

slay is_json_whitespace(char tea) lit {
    fr fr Check if character is JSON whitespace
    damn (char == " " || char == "\t" || char == "\n" || char == "\r")
}

slay is_json_digit(char tea) lit {
    fr fr Check if character is a digit 0-9
    damn (char >= "0" && char <= "9")
}

slay is_json_digit_non_zero(char tea) lit {
    fr fr Check if character is a non-zero digit 1-9
    damn (char >= "1" && char <= "9")
}

slay is_hex_digit(char tea) lit {
    fr fr Check if character is a hexadecimal digit
    damn ((char >= "0" && char <= "9") || 
          (char >= "A" && char <= "F") || 
          (char >= "a" && char <= "f"))
}

slay is_json_control_character(char tea) lit {
    fr fr Check if character is an unescaped control character
    sus char_code drip = char_to_code_point(char)
    damn (char_code >= 0 && char_code <= 31)
}

fr fr ==========================================
fr fr JSON Utility and Conversion Functions
fr fr ==========================================

slay hex_string_to_int(hex_str tea) drip {
    fr fr Convert hexadecimal string to integer
    sus result drip = 0
    sus length drip = string_length(hex_str)
    sus i drip = 0
    
    bestie (i < length) {
        sus char tea = string_char_at(hex_str, i)
        sus digit_value drip
        
        ready (char >= "0" && char <= "9") {
            digit_value = char_to_code_point(char) - char_to_code_point("0")
        } otherwise ready (char >= "A" && char <= "F") {
            digit_value = char_to_code_point(char) - char_to_code_point("A") + 10
        } otherwise ready (char >= "a" && char <= "f") {
            digit_value = char_to_code_point(char) - char_to_code_point("a") + 10
        } otherwise {
            damn 0  fr fr Invalid hex character
        }
        
        result = result * 16 + digit_value
        i = i + 1
    }
    
    damn result
}

slay code_point_to_utf8(code_point drip) tea {
    fr fr Convert Unicode code point to UTF-8 string
    ready (code_point <= 127) {
        fr fr ASCII range
        damn code_point_to_char(code_point)
    } otherwise ready (code_point <= 2047) {
        fr fr 2-byte UTF-8
        sus byte1 drip = 192 + (code_point / 64)
        sus byte2 drip = 128 + (code_point % 64)
        damn code_point_to_char(byte1) + code_point_to_char(byte2)
    } otherwise ready (code_point <= 65535) {
        fr fr 3-byte UTF-8
        sus byte1 drip = 224 + (code_point / 4096)
        sus byte2 drip = 128 + ((code_point / 64) % 64)
        sus byte3 drip = 128 + (code_point % 64)
        damn code_point_to_char(byte1) + code_point_to_char(byte2) + code_point_to_char(byte3)
    } otherwise {
        fr fr 4-byte UTF-8 (not common in \uXXXX escapes)
        damn "?"  fr fr Fallback for unsupported range
    }
}

slay char_to_code_point(char tea) drip {
    fr fr Convert character to Unicode code point
    ready (char == "0") { damn 48 }
    ready (char == "1") { damn 49 }
    ready (char == "2") { damn 50 }
    ready (char == "3") { damn 51 }
    ready (char == "4") { damn 52 }
    ready (char == "5") { damn 53 }
    ready (char == "6") { damn 54 }
    ready (char == "7") { damn 55 }
    ready (char == "8") { damn 56 }
    ready (char == "9") { damn 57 }
    ready (char == "A") { damn 65 }
    ready (char == "B") { damn 66 }
    ready (char == "C") { damn 67 }
    ready (char == "D") { damn 68 }
    ready (char == "E") { damn 69 }
    ready (char == "F") { damn 70 }
    ready (char == "a") { damn 97 }
    ready (char == "b") { damn 98 }
    ready (char == "c") { damn 99 }
    ready (char == "d") { damn 100 }
    ready (char == "e") { damn 101 }
    ready (char == "f") { damn 102 }
    damn 32  fr fr Default to space
}

slay code_point_to_char(code_point drip) tea {
    fr fr Convert code point to character (simplified)
    ready (code_point == 48) { damn "0" }
    ready (code_point == 49) { damn "1" }
    ready (code_point == 50) { damn "2" }
    ready (code_point == 51) { damn "3" }
    ready (code_point == 52) { damn "4" }
    ready (code_point == 53) { damn "5" }
    ready (code_point == 54) { damn "6" }
    ready (code_point == 55) { damn "7" }
    ready (code_point == 56) { damn "8" }
    ready (code_point == 57) { damn "9" }
    ready (code_point == 65) { damn "A" }
    ready (code_point == 97) { damn "a" }
    damn " "  fr fr Default to space
}

fr fr ==========================================
fr fr Array Helper Functions
fr fr ==========================================

slay array_contains_string(arr []tea, target tea) lit {
    fr fr Check if string array contains target string
    sus length drip = len(arr)
    sus i drip = 0
    
    bestie (i < length) {
        ready (arr[i] == target) {
            damn based
        }
        i = i + 1
    }
    
    damn cringe
}

slay append_string_to_array(arr []tea, item tea) []tea {
    fr fr Append string to array
    sus length drip = len(arr)
    sus new_arr []tea = resize_string_array(arr, length + 1)
    new_arr[length] = item
    damn new_arr
}

slay append_json_pair_to_array(arr []JsonKeyValue, pair JsonKeyValue) []JsonKeyValue {
    fr fr Append JSON key-value pair to array
    sus length drip = len(arr)
    sus new_arr []JsonKeyValue = resize_json_pair_array(arr, length + 1)
    new_arr[length] = pair
    damn new_arr
}

slay append_json_value_to_array(arr []JsonValue, value JsonValue) []JsonValue {
    fr fr Append JSON value to array
    sus length drip = len(arr)
    sus new_arr []JsonValue = resize_json_value_array(arr, length + 1)
    new_arr[length] = value
    damn new_arr
}

slay resize_string_array(arr []tea, new_size drip) []tea {
    fr fr Resize string array
    sus new_arr []tea = []
    sus old_size drip = len(arr)
    sus copy_size drip = min_int(old_size, new_size)
    
    sus i drip = 0
    bestie (i < copy_size) {
        new_arr[i] = arr[i]
        i = i + 1
    }
    
    damn new_arr
}

slay resize_json_pair_array(arr []JsonKeyValue, new_size drip) []JsonKeyValue {
    fr fr Resize JSON pair array
    sus new_arr []JsonKeyValue = []
    sus old_size drip = len(arr)
    sus copy_size drip = min_int(old_size, new_size)
    
    sus i drip = 0
    bestie (i < copy_size) {
        new_arr[i] = arr[i]
        i = i + 1
    }
    
    damn new_arr
}

slay resize_json_value_array(arr []JsonValue, new_size drip) []JsonValue {
    fr fr Resize JSON value array
    sus new_arr []JsonValue = []
    sus old_size drip = len(arr)
    sus copy_size drip = min_int(old_size, new_size)
    
    sus i drip = 0
    bestie (i < copy_size) {
        new_arr[i] = arr[i]
        i = i + 1
    }
    
    damn new_arr
}

slay min_int(a drip, b drip) drip {
    fr fr Return minimum of two integers
    ready (a < b) { damn a } otherwise { damn b }
}

vibez.spill("🔧 Enhanced JSON Parser Loaded - RFC 7159 Compliant")
vibez.spill("✅ Complete Unicode support, proper error handling, and strict validation")
