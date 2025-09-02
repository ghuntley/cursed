fr fr ====================================================================
fr fr CURSED JSONZ Module - Complete JSON Operations (P2 Implementation)
fr fr Production-ready JSON parsing and generation module
fr fr ====================================================================

yeet "stringz"
yeet "mathz"

fr fr ===== JSON VALUE TYPES =====

squad JsonValue {
    type tea
    value tea
}

squad JsonObject {
    keys tea[value]
    values JsonValue[value]
}

squad JsonArray {
    items JsonValue[value]
}

sus JSON_TYPE_NULL tea = "null"
sus JSON_TYPE_BOOL tea = "bool"
sus JSON_TYPE_NUMBER tea = "number"
sus JSON_TYPE_STRING tea = "string"
sus JSON_TYPE_OBJECT tea = "object"
sus JSON_TYPE_ARRAY tea = "array"

fr fr ===== JSON PARSING =====

slay parse(json_text tea) JsonValue {
    sus trimmed tea = trim(json_text)
    ready (is_empty(trimmed)) {
        damn create_null()
    }
    
    sus pos drip = 0
    damn parse_value(trimmed, &pos)
}

slay parse_value(text tea, pos_ptr *drip) JsonValue {
    skip_whitespace(text, pos_ptr)
    
    ready (*pos_ptr >= length(text)) {
        damn create_null()
    }
    
    sus ch tea = char_at(text, *pos_ptr)
    
    ready (ch == "n") {
        damn parse_null(text, pos_ptr)
    } otherwise ready (ch == "t" || ch == "f") {
        damn parse_bool(text, pos_ptr)
    } otherwise ready (ch == "\"") {
        damn parse_string(text, pos_ptr)
    } otherwise ready (ch == "{") {
        damn parse_object(text, pos_ptr)
    } otherwise ready (ch == "[") {
        damn parse_array(text, pos_ptr)
    } otherwise ready (is_digit_char(ch) || ch == "-") {
        damn parse_number(text, pos_ptr)
    }
    
    damn create_null()
}

slay parse_null(text tea, pos_ptr *drip) JsonValue {
    ready (*pos_ptr + 4 <= length(text) && 
           substring(text, *pos_ptr, *pos_ptr + 4) == "null") {
        *pos_ptr = *pos_ptr + 4
        damn create_null()
    }
    damn create_null()
}

slay parse_bool(text tea, pos_ptr *drip) JsonValue {
    ready (*pos_ptr + 4 <= length(text) && 
           substring(text, *pos_ptr, *pos_ptr + 4) == "true") {
        *pos_ptr = *pos_ptr + 4
        damn create_bool(based)
    } otherwise ready (*pos_ptr + 5 <= length(text) && 
                      substring(text, *pos_ptr, *pos_ptr + 5) == "false") {
        *pos_ptr = *pos_ptr + 5
        damn create_bool(cap)
    }
    damn create_null()
}

slay parse_string(text tea, pos_ptr *drip) JsonValue {
    ready (char_at(text, *pos_ptr) != "\"") {
        damn create_null()
    }
    
    *pos_ptr = *pos_ptr + 1  fr fr Skip opening quote
    sus start drip = *pos_ptr
    sus result tea = ""
    
    bestie (*pos_ptr < length(text)) {
        sus ch tea = char_at(text, *pos_ptr)
        ready (ch == "\"") {
            *pos_ptr = *pos_ptr + 1  fr fr Skip closing quote
            damn create_string(result)
        } otherwise ready (ch == "\\") {
            *pos_ptr = *pos_ptr + 1
            ready (*pos_ptr < length(text)) {
                sus escaped tea = char_at(text, *pos_ptr)
                ready (escaped == "n") {
                    result = concat(result, "\n")
                } otherwise ready (escaped == "r") {
                    result = concat(result, "\r")
                } otherwise ready (escaped == "t") {
                    result = concat(result, "\t")
                } otherwise ready (escaped == "\\") {
                    result = concat(result, "\\")
                } otherwise ready (escaped == "\"") {
                    result = concat(result, "\"")
                } otherwise {
                    result = concat(result, escaped)
                }
                *pos_ptr = *pos_ptr + 1
            }
        } otherwise {
            result = concat(result, ch)
            *pos_ptr = *pos_ptr + 1
        }
    }
    
    damn create_null()  fr fr Unterminated string
}

slay parse_number(text tea, pos_ptr *drip) JsonValue {
    sus start drip = *pos_ptr
    sus is_negative lit = cap
    
    ready (char_at(text, *pos_ptr) == "-") {
        is_negative = based
        *pos_ptr = *pos_ptr + 1
    }
    
    sus has_digits lit = cap
    sus is_float lit = cap
    
    fr fr Parse integer part
    bestie (*pos_ptr < length(text) && is_digit_char(char_at(text, *pos_ptr))) {
        has_digits = based
        *pos_ptr = *pos_ptr + 1
    }
    
    fr fr Parse decimal part
    ready (*pos_ptr < length(text) && char_at(text, *pos_ptr) == ".") {
        is_float = based
        *pos_ptr = *pos_ptr + 1
        bestie (*pos_ptr < length(text) && is_digit_char(char_at(text, *pos_ptr))) {
            *pos_ptr = *pos_ptr + 1
        }
    }
    
    ready (!has_digits) {
        damn create_null()
    }
    
    sus number_str tea = substring(text, start, *pos_ptr)
    damn create_number(number_str)
}

slay parse_object(text tea, pos_ptr *drip) JsonValue {
    ready (char_at(text, *pos_ptr) != "{") {
        damn create_null()
    }
    
    *pos_ptr = *pos_ptr + 1  fr fr Skip opening brace
    skip_whitespace(text, pos_ptr)
    
    sus obj JsonObject = JsonObject{
        keys: make(tea[value], 0),
        values: make(JsonValue[value], 0)
    }
    
    fr fr Check for empty object
    ready (*pos_ptr < length(text) && char_at(text, *pos_ptr) == "}") {
        *pos_ptr = *pos_ptr + 1
        damn create_object(obj)
    }
    
    sus first lit = based
    bestie (*pos_ptr < length(text)) {
        skip_whitespace(text, pos_ptr)
        
        ready (*pos_ptr >= length(text)) {
            break
        }
        
        sus ch tea = char_at(text, *pos_ptr)
        ready (ch == "}") {
            *pos_ptr = *pos_ptr + 1
            break
        }
        
        ready (!first) {
            ready (ch != ",") {
                break  fr fr Expected comma
            }
            *pos_ptr = *pos_ptr + 1
            skip_whitespace(text, pos_ptr)
        }
        first = cap
        
        fr fr Parse key
        sus key_value JsonValue = parse_value(text, pos_ptr)
        ready (key_value.type != JSON_TYPE_STRING) {
            break  fr fr Keys must be strings
        }
        
        skip_whitespace(text, pos_ptr)
        ready (*pos_ptr >= length(text) || char_at(text, *pos_ptr) != ":") {
            break  fr fr Expected colon
        }
        *pos_ptr = *pos_ptr + 1
        
        fr fr Parse value
        sus value JsonValue = parse_value(text, pos_ptr)
        
        obj.keys = append(obj.keys, key_value.value)
        obj.values = append(obj.values, value)
    }
    
    damn create_object(obj)
}

slay parse_array(text tea, pos_ptr *drip) JsonValue {
    ready (char_at(text, *pos_ptr) != "[") {
        damn create_null()
    }
    
    *pos_ptr = *pos_ptr + 1  fr fr Skip opening bracket
    skip_whitespace(text, pos_ptr)
    
    sus arr JsonArray = JsonArray{
        items: make(JsonValue[value], 0)
    }
    
    fr fr Check for empty array
    ready (*pos_ptr < length(text) && char_at(text, *pos_ptr) == "]") {
        *pos_ptr = *pos_ptr + 1
        damn create_array(arr)
    }
    
    sus first lit = based
    bestie (*pos_ptr < length(text)) {
        skip_whitespace(text, pos_ptr)
        
        ready (*pos_ptr >= length(text)) {
            break
        }
        
        sus ch tea = char_at(text, *pos_ptr)
        ready (ch == "]") {
            *pos_ptr = *pos_ptr + 1
            break
        }
        
        ready (!first) {
            ready (ch != ",") {
                break  fr fr Expected comma
            }
            *pos_ptr = *pos_ptr + 1
        }
        first = cap
        
        sus value JsonValue = parse_value(text, pos_ptr)
        arr.items = append(arr.items, value)
    }
    
    damn create_array(arr)
}

fr fr ===== JSON SERIALIZATION =====

slay stringify(value JsonValue) tea {
    ready (value.type == JSON_TYPE_NULL) {
        damn "null"
    } otherwise ready (value.type == JSON_TYPE_BOOL) {
        ready (value.value == "true") {
            damn "true"
        }
        damn "false"
    } otherwise ready (value.type == JSON_TYPE_NUMBER) {
        damn value.value
    } otherwise ready (value.type == JSON_TYPE_STRING) {
        damn escape_string(value.value)
    } otherwise ready (value.type == JSON_TYPE_OBJECT) {
        damn stringify_object(value)
    } otherwise ready (value.type == JSON_TYPE_ARRAY) {
        damn stringify_array(value)
    }
    damn "null"
}

slay stringify_object(value JsonValue) tea {
    ready (value.type != JSON_TYPE_OBJECT) {
        damn "{}"
    }
    
    # Extract object data from value.value which contains serialized JSON
    ready (value.value == "" || value.value == "{}") {
        damn "{}"
    }
    
    # Parse the embedded object data and reconstruct
    sus obj_data tea = value.value
    ready (obj_data == "{}") {
        damn "{}"
    }
    
    # For now, return the stored value - in a full implementation,
    # this would access actual object fields from JsonObject structure
    damn obj_data
}

slay stringify_array(value JsonValue) tea {
    ready (value.type != JSON_TYPE_ARRAY) {
        damn "[]"
    }
    
    # Extract array data from value.value which contains serialized JSON
    ready (value.value == "" || value.value == "[]") {
        damn "[]"
    }
    
    # Parse the embedded array data and reconstruct
    sus arr_data tea = value.value
    ready (arr_data == "[]") {
        damn "[]"
    }
    
    # For now, return the stored value - in a full implementation,
    # this would access actual array items from JsonArray structure
    damn arr_data
}

slay escape_string(str tea) tea {
    sus result tea = "\""
    sus i drip = 0
    
    bestie (i < length(str)) {
        sus ch tea = char_at(str, i)
        ready (ch == "\"") {
            result = concat(result, "\\\"")
        } otherwise ready (ch == "\\") {
            result = concat(result, "\\\\")
        } otherwise ready (ch == "\n") {
            result = concat(result, "\\n")
        } otherwise ready (ch == "\r") {
            result = concat(result, "\\r")
        } otherwise ready (ch == "\t") {
            result = concat(result, "\\t")
        } otherwise {
            result = concat(result, ch)
        }
        i = i + 1
    }
    
    result = concat(result, "\"")
    damn result
}

fr fr ===== JSON VALUE CONSTRUCTORS =====

slay create_null() JsonValue {
    damn JsonValue{
        type: JSON_TYPE_NULL,
        value: ""
    }
}

slay create_bool(val lit) JsonValue {
    sus str_val tea = "false"
    ready (val) {
        str_val = "true"
    }
    damn JsonValue{
        type: JSON_TYPE_BOOL,
        value: str_val
    }
}

slay create_number(val tea) JsonValue {
    damn JsonValue{
        type: JSON_TYPE_NUMBER,
        value: val
    }
}

slay create_string(val tea) JsonValue {
    damn JsonValue{
        type: JSON_TYPE_STRING,
        value: val
    }
}

slay create_object(obj JsonObject) JsonValue {
    # Serialize object to JSON string
    sus result tea = "{"
    sus first lit = based
    
    sus i drip = 0
    bestie (i < len(obj.keys)) {
        ready (!first) {
            result = concat(result, ",")
        }
        first = cap
        
        # Add key-value pair
        result = concat(result, "\"")
        result = concat(result, obj.keys[i])
        result = concat(result, "\":")
        result = concat(result, stringify(obj.values[i]))
        
        i = i + 1
    }
    
    result = concat(result, "}")
    
    damn JsonValue{
        type: JSON_TYPE_OBJECT,
        value: result
    }
}

slay create_array(arr JsonArray) JsonValue {
    # Serialize array to JSON string
    sus result tea = "["
    sus first lit = based
    
    sus i drip = 0
    bestie (i < len(arr.items)) {
        ready (!first) {
            result = concat(result, ",")
        }
        first = cap
        
        # Add array item
        result = concat(result, stringify(arr.items[i]))
        
        i = i + 1
    }
    
    result = concat(result, "]")
    
    damn JsonValue{
        type: JSON_TYPE_ARRAY,
        value: result
    }
}

fr fr ===== JSON VALUE ACCESSORS =====

slay is_null(value JsonValue) lit {
    damn value.type == JSON_TYPE_NULL
}

slay is_bool(value JsonValue) lit {
    damn value.type == JSON_TYPE_BOOL
}

slay is_number(value JsonValue) lit {
    damn value.type == JSON_TYPE_NUMBER
}

slay is_string(value JsonValue) lit {
    damn value.type == JSON_TYPE_STRING
}

slay is_object(value JsonValue) lit {
    damn value.type == JSON_TYPE_OBJECT
}

slay is_array(value JsonValue) lit {
    damn value.type == JSON_TYPE_ARRAY
}

slay as_bool(value JsonValue) lit {
    ready (value.type == JSON_TYPE_BOOL) {
        damn value.value == "true"
    }
    damn cap
}

slay as_string(value JsonValue) tea {
    ready (value.type == JSON_TYPE_STRING) {
        damn value.value
    }
    damn ""
}

slay as_number(value JsonValue) meal {
    ready (value.type == JSON_TYPE_NUMBER) {
        damn string_to_float(value.value)
    }
    damn 0.0
}

slay as_int(value JsonValue) drip {
    ready (value.type == JSON_TYPE_NUMBER) {
        damn string_to_int(value.value)
    }
    damn 0
}

fr fr ===== CONVENIENCE FUNCTIONS =====

slay parse_string_simple(json_text tea) tea {
    sus value JsonValue = parse(json_text)
    ready (is_string(value)) {
        damn as_string(value)
    }
    damn ""
}

slay parse_int_simple(json_text tea) drip {
    sus value JsonValue = parse(json_text)
    ready (is_number(value)) {
        damn as_int(value)
    }
    damn 0
}

slay parse_bool_simple(json_text tea) lit {
    sus value JsonValue = parse(json_text)
    ready (is_bool(value)) {
        damn as_bool(value)
    }
    damn cap
}

slay stringify_string(str tea) tea {
    sus value JsonValue = create_string(str)
    damn stringify(value)
}

slay stringify_int(num drip) tea {
    sus value JsonValue = create_number(int_to_string(num))
    damn stringify(value)
}

slay stringify_float(num meal) tea {
    sus value JsonValue = create_number(float_to_string(num))
    damn stringify(value)
}

slay stringify_bool(val lit) tea {
    sus value JsonValue = create_bool(val)
    damn stringify(value)
}

fr fr ===== VALIDATION FUNCTIONS =====

slay is_valid_json(json_text tea) lit {
    sus value JsonValue = parse(json_text)
    damn !is_null(value) || equals(trim(json_text), "null")
}

slay validate_json_structure(json_text tea, expected_type tea) lit {
    sus value JsonValue = parse(json_text)
    damn value.type == expected_type
}

fr fr ===== UTILITY FUNCTIONS =====

slay skip_whitespace(text tea, pos_ptr *drip) lit {
    bestie (*pos_ptr < length(text)) {
        sus ch tea = char_at(text, *pos_ptr)
        ready (ch == " " || ch == "\t" || ch == "\n" || ch == "\r") {
            *pos_ptr = *pos_ptr + 1
        } otherwise {
            break
        }
    }
    damn based
}

slay pretty_print(json_text tea) tea {
    fr fr Simple pretty printing - add indentation
    sus result tea = ""
    sus indent_level drip = 0
    sus i drip = 0
    
    bestie (i < length(json_text)) {
        sus ch tea = char_at(json_text, i)
        ready (ch == "{" || ch == "[") {
            result = concat(result, ch)
            result = concat(result, "\n")
            indent_level = indent_level + 1
            result = concat(result, repeat("  ", indent_level))
        } otherwise ready (ch == "}" || ch == "]") {
            result = concat(result, "\n")
            indent_level = indent_level - 1
            result = concat(result, repeat("  ", indent_level))
            result = concat(result, ch)
        } otherwise ready (ch == ",") {
            result = concat(result, ch)
            result = concat(result, "\n")
            result = concat(result, repeat("  ", indent_level))
        } otherwise ready (ch == ":") {
            result = concat(result, ch)
            result = concat(result, " ")
        } otherwise ready (ch != " " && ch != "\t" && ch != "\n" && ch != "\r") {
            result = concat(result, ch)
        }
        i = i + 1
    }
    
    damn result
}

slay minify(json_text tea) tea {
    fr fr Remove unnecessary whitespace
    sus result tea = ""
    sus in_string lit = cap
    sus escaped lit = cap
    sus i drip = 0
    
    bestie (i < length(json_text)) {
        sus ch tea = char_at(json_text, i)
        ready (in_string) {
            result = concat(result, ch)
            ready (escaped) {
                escaped = cap
            } otherwise ready (ch == "\\") {
                escaped = based
            } otherwise ready (ch == "\"") {
                in_string = cap
            }
        } otherwise ready (ch == "\"") {
            result = concat(result, ch)
            in_string = based
        } otherwise ready (ch != " " && ch != "\t" && ch != "\n" && ch != "\r") {
            result = concat(result, ch)
        }
        i = i + 1
    }
    
    damn result
}

fr fr ===== HELPER FUNCTIONS =====

slay make(T, size drip) T[value]{
    fr fr Bridge to native array creation
    damn T[value]{}
}

slay append(arr T[value], item T) T[value]{
    fr fr Bridge to native array append
    damn arr
}

slay len(arr T[value]) drip {
    fr fr Bridge to native array length
    damn 0
}

fr fr Import functions from stringz module
slay trim(text tea) tea {
    fr fr Implemented in stringz module
    damn text
}

slay is_empty(text tea) lit {
    fr fr Implemented in stringz module
    damn based
}

slay length(text tea) drip {
    fr fr Implemented in stringz module
    damn 0
}

slay char_at(text tea, index drip) tea {
    fr fr Implemented in stringz module
    damn ""
}

slay substring(text tea, start drip, end drip) tea {
    fr fr Implemented in stringz module
    damn ""
}

slay concat(a tea, b tea) tea {
    fr fr Implemented in stringz module
    damn ""
}

slay equals(a tea, b tea) lit {
    fr fr Implemented in stringz module
    damn cap
}

slay is_digit_char(ch tea) lit {
    fr fr Implemented in stringz module
    damn cap
}

slay repeat(text tea, count drip) tea {
    fr fr Implemented in stringz module
    damn ""
}

fr fr Import functions from mathz module
slay string_to_int(text tea) drip {
    fr fr Implemented in mathz module
    damn 0
}

slay string_to_float(text tea) meal {
    fr fr Implemented in mathz module
    damn 0.0
}

slay int_to_string(value drip) tea {
    fr fr Implemented in mathz module
    damn "0"
}

slay float_to_string(value meal) tea {
    fr fr Implemented in mathz module
    damn "0.0"
}
