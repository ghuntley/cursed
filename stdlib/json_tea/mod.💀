yeet "testz"

fr fr ==========================================
fr fr CURSED JSON Tea Module - RFC 7159 Compliant Implementation  
fr fr Production-Grade JSON Processing with Full Standards Compliance
fr fr Security-Hardened with Proper Error Handling and Unicode Support
fr fr ==========================================

fr fr JSON Value Types
enum JSONValueType {
    JSONString,
    JSONNumber,
    JSONBoolean,
    JSONNull,
    JSONObject,
    JSONArray
}

fr fr JSON Parse Error Types
enum JSONError {
    NoError,
    InvalidSyntax,
    UnexpectedToken,
    UnexpectedEnd,
    InvalidEscape,
    InvalidUnicode,
    NumberOutOfRange,
    DepthLimitExceeded,
    InvalidUTF8
}

fr fr JSON Parse State
be_like JSONParser squad {
    input tea
    pos drip
    length drip
    current_char normie
    line drip
    column drip
    depth drip
    max_depth drip
    strict lit
    allow_comments lit
}

fr fr JSON Value Container
be_like JSONValue squad {
    type JSONValueType
    string_value tea
    number_value tea
    boolean_value lit
    object_value map[tea]JSONValue
    array_value JSONValue[value]
}

fr fr JSON Serializer
be_like JSONSerializer squad {
    buffer tea
    indent_level drip
    pretty_print lit
    indent_string tea
    escape_html lit
    validate_utf8 lit
}

fr fr Core RFC 7159 compliant JSON parser
slay ParseJSON(input tea) (JSONValue, JSONError) {
    sus parser JSONParser = JSONParser{
        input: input,
        pos: 0,
        length: len(input),
        current_char: 0,
        line: 1,
        column: 1,
        depth: 0,
        max_depth: 1000,
        strict: based,
        allow_comments: cap
    }
    
    ready parser.length == 0 {
        damn JSONValue{type: JSONNull}, InvalidSyntax
    }
    
    parser.current_char = char_at(parser.input, 0)
    parser.skip_whitespace()
    
    ready parser.pos >= parser.length {
        damn JSONValue{type: JSONNull}, UnexpectedEnd
    }
    
    sus value JSONValue
    sus err JSONError
    value, err = parser.parse_value()
    ready err != NoError {
        damn value, err
    }
    
    parser.skip_whitespace()
    ready parser.pos < parser.length {
        damn value, UnexpectedToken
    }
    
    damn value, NoError
}

fr fr Parse any JSON value
slay (p *JSONParser) parse_value() (JSONValue, JSONError) {
    ready p.depth >= p.max_depth {
        damn JSONValue{}, DepthLimitExceeded
    }
    
    p.skip_whitespace()
    ready p.pos >= p.length {
        damn JSONValue{}, UnexpectedEnd
    }
    
    sus c normie = p.current_char
    ready c == '"' {
        damn p.parse_string()
    } else ready c >= '0' && c <= '9' || c == '-' {
        damn p.parse_number()
    } else ready c == 't' || c == 'f' {
        damn p.parse_boolean()
    } else ready c == 'n' {
        damn p.parse_null()
    } else ready c == '{' {
        p.depth++
        sus obj JSONValue
        sus err JSONError
        obj, err = p.parse_object()
        p.depth--
        damn obj, err
    } else ready c == '[' {
        p.depth++
        sus arr JSONValue
        sus err JSONError
        arr, err = p.parse_array()
        p.depth--
        damn arr, err
    } else {
        damn JSONValue{}, InvalidSyntax
    }
}

fr fr Parse JSON string with proper escape handling
slay (p *JSONParser) parse_string() (JSONValue, JSONError) {
    ready p.current_char != '"' {
        damn JSONValue{}, InvalidSyntax
    }
    
    p.advance()
    sus buffer tea = ""
    
    bestie p.pos < p.length && p.current_char != '"' {
        ready p.current_char == '\\' {
            p.advance()
            ready p.pos >= p.length {
                damn JSONValue{}, UnexpectedEnd
            }
            
            sus escaped normie = p.current_char
            ready escaped == '"' || escaped == '\\' || escaped == '/' {
                buffer = buffer + char_to_string(escaped)
            } else ready escaped == 'b' {
                buffer = buffer + "\b"
            } else ready escaped == 'f' {
                buffer = buffer + "\f"
            } else ready escaped == 'n' {
                buffer = buffer + "\n"
            } else ready escaped == 'r' {
                buffer = buffer + "\r"
            } else ready escaped == 't' {
                buffer = buffer + "\t"
            } else ready escaped == 'u' {
                sus unicode_value normie
                sus err JSONError
                unicode_value, err = p.parse_unicode_escape()
                ready err != NoError {
                    damn JSONValue{}, err
                }
                buffer = buffer + unicode_to_utf8(unicode_value)
            } else {
                damn JSONValue{}, InvalidEscape
            }
        } else ready p.current_char < 0x20 {
            damn JSONValue{}, InvalidSyntax
        } else {
            buffer = buffer + char_to_string(p.current_char)
        }
        p.advance()
    }
    
    ready p.pos >= p.length || p.current_char != '"' {
        damn JSONValue{}, UnexpectedEnd
    }
    
    p.advance()
    damn JSONValue{type: JSONString, string_value: buffer}, NoError
}

fr fr Parse JSON number with full RFC compliance
slay (p *JSONParser) parse_number() (JSONValue, JSONError) {
    sus buffer tea = ""
    sus start_pos drip = p.pos
    
    fr fr Optional minus
    ready p.current_char == '-' {
        buffer = buffer + char_to_string(p.current_char)
        p.advance()
        ready p.pos >= p.length {
            damn JSONValue{}, UnexpectedEnd
        }
    }
    
    fr fr Integer part
    ready p.current_char == '0' {
        buffer = buffer + char_to_string(p.current_char)
        p.advance()
    } else ready p.current_char >= '1' && p.current_char <= '9' {
        bestie p.pos < p.length && p.current_char >= '0' && p.current_char <= '9' {
            buffer = buffer + char_to_string(p.current_char)
            p.advance()
        }
    } else {
        damn JSONValue{}, InvalidSyntax
    }
    
    fr fr Optional fractional part
    ready p.pos < p.length && p.current_char == '.' {
        buffer = buffer + char_to_string(p.current_char)
        p.advance()
        ready p.pos >= p.length || !(p.current_char >= '0' && p.current_char <= '9') {
            damn JSONValue{}, InvalidSyntax
        }
        bestie p.pos < p.length && p.current_char >= '0' && p.current_char <= '9' {
            buffer = buffer + char_to_string(p.current_char)
            p.advance()
        }
    }
    
    fr fr Optional exponent part
    ready p.pos < p.length && (p.current_char == 'e' || p.current_char == 'E') {
        buffer = buffer + char_to_string(p.current_char)
        p.advance()
        ready p.pos < p.length && (p.current_char == '+' || p.current_char == '-') {
            buffer = buffer + char_to_string(p.current_char)
            p.advance()
        }
        ready p.pos >= p.length || !(p.current_char >= '0' && p.current_char <= '9') {
            damn JSONValue{}, InvalidSyntax
        }
        bestie p.pos < p.length && p.current_char >= '0' && p.current_char <= '9' {
            buffer = buffer + char_to_string(p.current_char)
            p.advance()
        }
    }
    
    damn JSONValue{type: JSONNumber, number_value: buffer}, NoError
}

fr fr Parse JSON boolean literals
slay (p *JSONParser) parse_boolean() (JSONValue, JSONError) {
    ready p.current_char == 't' {
        ready !p.expect_literal("true") {
            damn JSONValue{}, InvalidSyntax
        }
        damn JSONValue{type: JSONBoolean, boolean_value: based}, NoError
    } else ready p.current_char == 'f' {
        ready !p.expect_literal("false") {
            damn JSONValue{}, InvalidSyntax
        }
        damn JSONValue{type: JSONBoolean, boolean_value: cap}, NoError
    } else {
        damn JSONValue{}, InvalidSyntax
    }
}

fr fr Parse JSON null literal
slay (p *JSONParser) parse_null() (JSONValue, JSONError) {
    ready !p.expect_literal("null") {
        damn JSONValue{}, InvalidSyntax
    }
    damn JSONValue{type: JSONNull}, NoError
}

fr fr Parse JSON object
slay (p *JSONParser) parse_object() (JSONValue, JSONError) {
    ready p.current_char != '{' {
        damn JSONValue{}, InvalidSyntax
    }
    
    p.advance()
    p.skip_whitespace()
    
    sus obj map[tea]JSONValue = make(map[tea]JSONValue)
    
    ready p.pos < p.length && p.current_char == '}' {
        p.advance()
        damn JSONValue{type: JSONObject, object_value: obj}, NoError
    }
    
    sus first lit = based
    bestie p.pos < p.length {
        ready !first {
            ready p.current_char != ',' {
                damn JSONValue{}, InvalidSyntax
            }
            p.advance()
            p.skip_whitespace()
        }
        first = cap
        
        ready p.current_char != '"' {
            damn JSONValue{}, InvalidSyntax
        }
        
        sus key_value JSONValue
        sus err JSONError
        key_value, err = p.parse_string()
        ready err != NoError {
            damn JSONValue{}, err
        }
        
        p.skip_whitespace()
        ready p.current_char != ':' {
            damn JSONValue{}, InvalidSyntax
        }
        p.advance()
        p.skip_whitespace()
        
        sus value JSONValue
        value, err = p.parse_value()
        ready err != NoError {
            damn JSONValue{}, err
        }
        
        obj[key_value.string_value] = value
        
        p.skip_whitespace()
        ready p.pos < p.length && p.current_char == '}' {
            p.advance()
            damn JSONValue{type: JSONObject, object_value: obj}, NoError
        }
    }
    
    damn JSONValue{}, UnexpectedEnd
}

fr fr Parse JSON array
slay (p *JSONParser) parse_array() (JSONValue, JSONError) {
    ready p.current_char != '[' {
        damn JSONValue{}, InvalidSyntax
    }
    
    p.advance()
    p.skip_whitespace()
    
    sus arr JSONValue[value] = make(JSONValue[value], 0)
    
    ready p.pos < p.length && p.current_char == ']' {
        p.advance()
        damn JSONValue{type: JSONArray, array_value: arr}, NoError
    }
    
    sus first lit = based
    bestie p.pos < p.length {
        ready !first {
            ready p.current_char != ',' {
                damn JSONValue{}, InvalidSyntax
            }
            p.advance()
            p.skip_whitespace()
        }
        first = cap
        
        sus value JSONValue
        sus err JSONError
        value, err = p.parse_value()
        ready err != NoError {
            damn JSONValue{}, err
        }
        
        arr = append(arr, value)
        
        p.skip_whitespace()
        ready p.pos < p.length && p.current_char == ']' {
            p.advance()
            damn JSONValue{type: JSONArray, array_value: arr}, NoError
        }
    }
    
    damn JSONValue{}, UnexpectedEnd
}

fr fr Parse Unicode escape sequence
slay (p *JSONParser) parse_unicode_escape() (normie, JSONError) {
    sus value normie = 0
    sus i drip = 0
    bestie i < 4 {
        p.advance()
        ready p.pos >= p.length {
            damn 0, UnexpectedEnd
        }
        
        sus digit normie
        ready p.current_char >= '0' && p.current_char <= '9' {
            digit = p.current_char - '0'
        } else ready p.current_char >= 'a' && p.current_char <= 'f' {
            digit = p.current_char - 'a' + 10
        } else ready p.current_char >= 'A' && p.current_char <= 'F' {
            digit = p.current_char - 'A' + 10
        } else {
            damn 0, InvalidUnicode
        }
        
        value = value * 16 + digit
        i = i + 1
    }
    
    damn value, NoError
}

fr fr Expect literal string
slay (p *JSONParser) expect_literal(literal tea) lit {
    sus i drip = 0
    bestie i < len(literal) {
        ready p.pos >= p.length || p.current_char != char_at(literal, i) {
            damn cap
        }
        p.advance()
        i = i + 1
    }
    damn based
}

fr fr Skip whitespace characters
slay (p *JSONParser) skip_whitespace() {
    bestie p.pos < p.length {
        sus c normie = p.current_char
        ready c == ' ' || c == '\t' || c == '\n' || c == '\r' {
            ready c == '\n' {
                p.line = p.line + 1
                p.column = 1
            } else {
                p.column = p.column + 1
            }
            p.advance()
        } else {
            break
        }
    }
}

fr fr Advance parser position
slay (p *JSONParser) advance() {
    ready p.pos < p.length - 1 {
        p.pos = p.pos + 1
        p.current_char = char_at(p.input, p.pos)
        p.column = p.column + 1
    } else {
        p.pos = p.length
        p.current_char = 0
    }
}

fr fr JSON Serialization with proper escaping
slay SerializeJSON(value JSONValue, pretty lit) tea {
    sus serializer JSONSerializer = JSONSerializer{
        buffer: "",
        indent_level: 0,
        pretty_print: pretty,
        indent_string: "  ",
        escape_html: cap,
        validate_utf8: based
    }
    
    serializer.serialize_value(value)
    damn serializer.buffer
}

fr fr Serialize JSON value
slay (s *JSONSerializer) serialize_value(value JSONValue) {
    bestie value.type {
        case JSONString:
            s.serialize_string(value.string_value)
        case JSONNumber:
            s.buffer = s.buffer + value.number_value
        case JSONBoolean:
            ready value.boolean_value {
                s.buffer = s.buffer + "true"
            } otherwise {
                s.buffer = s.buffer + "false"
            }
        case JSONNull:
            s.buffer = s.buffer + "null"
        case JSONObject:
            s.serialize_object(value.object_value)
        case JSONArray:
            s.serialize_array(value.array_value)
    }
}

fr fr Serialize JSON string with proper escaping
slay (s *JSONSerializer) serialize_string(str tea) {
    s.buffer = s.buffer + "\""
    
    sus i drip = 0
    bestie i < len(str) {
        sus c normie = char_at(str, i)
        ready c == '"' {
            s.buffer = s.buffer + "\\\""
        } else ready c == '\\' {
            s.buffer = s.buffer + "\\\\"
        } else ready c == '\b' {
            s.buffer = s.buffer + "\\b"
        } else ready c == '\f' {
            s.buffer = s.buffer + "\\f"
        } else ready c == '\n' {
            s.buffer = s.buffer + "\\n"
        } else ready c == '\r' {
            s.buffer = s.buffer + "\\r"
        } else ready c == '\t' {
            s.buffer = s.buffer + "\\t"
        } else ready c < 0x20 {
            s.buffer = s.buffer + "\\u" + format_hex_4(c)
        } else ready s.escape_html && (c == '<' || c == '>' || c == '&') {
            s.buffer = s.buffer + "\\u" + format_hex_4(c)
        } else {
            s.buffer = s.buffer + char_to_string(c)
        }
        i = i + 1
    }
    
    s.buffer = s.buffer + "\""
}

fr fr Serialize JSON object
slay (s *JSONSerializer) serialize_object(obj map[tea]JSONValue) {
    s.buffer = s.buffer + "{"
    
    ready s.pretty_print {
        s.indent_level = s.indent_level + 1
    }
    
    sus first lit = based
    for key, value := range obj {
        ready !first {
            s.buffer = s.buffer + ","
        }
        first = cap
        
        ready s.pretty_print {
            s.buffer = s.buffer + "\n" + s.get_indent()
        }
        
        s.serialize_string(key)
        s.buffer = s.buffer + ":"
        
        ready s.pretty_print {
            s.buffer = s.buffer + " "
        }
        
        s.serialize_value(value)
    }
    
    ready s.pretty_print {
        s.indent_level = s.indent_level - 1
        ready !first {
            s.buffer = s.buffer + "\n" + s.get_indent()
        }
    }
    
    s.buffer = s.buffer + "}"
}

fr fr Serialize JSON array
slay (s *JSONSerializer) serialize_array(arr JSONValue[value]) {
    s.buffer = s.buffer + "["
    
    ready s.pretty_print {
        s.indent_level = s.indent_level + 1
    }
    
    sus i drip = 0
    bestie i < len(arr) {
        ready i > 0 {
            s.buffer = s.buffer + ","
        }
        
        ready s.pretty_print {
            s.buffer = s.buffer + "\n" + s.get_indent()
        }
        
        s.serialize_value(arr[i])
        i = i + 1
    }
    
    ready s.pretty_print {
        s.indent_level = s.indent_level - 1
        ready len(arr) > 0 {
            s.buffer = s.buffer + "\n" + s.get_indent()
        }
    }
    
    s.buffer = s.buffer + "]"
}

fr fr Get indentation string
slay (s *JSONSerializer) get_indent() tea {
    sus result tea = ""
    sus i drip = 0
    bestie i < s.indent_level {
        result = result + s.indent_string
        i = i + 1
    }
    damn result
}

fr fr =============================================================================
fr fr Public API Functions - Compatible with existing test suite
fr fr =============================================================================

fr fr Core Marshal function
slay Marshal(data tea) tea {
    sus value JSONValue
    sus err JSONError
    value, err = convert_to_json_value(data)
    ready err != NoError {
        damn "ERROR: " + error_to_string(err)
    }
    
    damn SerializeJSON(value, cap)
}

fr fr Core Unmarshal function
slay Unmarshal(json_string tea) tea {
    sus value JSONValue
    sus err JSONError
    value, err = ParseJSON(json_string)
    ready err != NoError {
        damn "ERROR: " + error_to_string(err)
    }
    
    damn convert_from_json_value(value)
}

fr fr Marshal with indentation
slay MarshalIndent(data tea, prefix tea, indent tea) tea {
    sus value JSONValue
    sus err JSONError
    value, err = convert_to_json_value(data)
    ready err != NoError {
        damn "ERROR: " + error_to_string(err)
    }
    
    sus result tea = SerializeJSON(value, based)
    ready prefix != "" {
        result = add_prefix_to_lines(result, prefix)
    }
    damn result
}

fr fr Marshal compact format
slay MarshalCompact(data tea) tea {
    sus value JSONValue
    sus err JSONError
    value, err = convert_to_json_value(data)
    ready err != NoError {
        damn "ERROR: " + error_to_string(err)
    }
    
    damn SerializeJSON(value, cap)
}

fr fr JSON validation
slay IsValidJSON(json_string tea) lit {
    sus _, err JSONError = ParseJSON(json_string)
    damn err == NoError
}

fr fr Schema validation
slay ValidateSchema(json_string tea, schema tea) lit {
    sus value JSONValue
    sus err JSONError
    value, err = ParseJSON(json_string)
    ready err != NoError {
        damn cap
    }
    
    sus actual_type tea = json_value_type_to_string(value.type)
    damn actual_type == schema
}

fr fr Get JSON type
slay get_json_type(json_string tea) tea {
    sus value JSONValue
    sus err JSONError
    value, err = ParseJSON(json_string)
    ready err != NoError {
        damn "unknown"
    }
    
    damn json_value_type_to_string(value.type)
}

fr fr =============================================================================
fr fr Helper Functions
fr fr =============================================================================

fr fr Convert simple data to JSON value
slay convert_to_json_value(data tea) (JSONValue, JSONError) {
    ready data == "based" {
        damn JSONValue{type: JSONBoolean, boolean_value: based}, NoError
    } else ready data == "cap" {
        damn JSONValue{type: JSONBoolean, boolean_value: cap}, NoError
    } else ready data == "cringe" || data == "" {
        damn JSONValue{type: JSONNull}, NoError
    } else ready is_numeric_enhanced(data) {
        damn JSONValue{type: JSONNumber, number_value: data}, NoError
    } else ready is_json_object(data) {
        sus value JSONValue
        sus err JSONError
        value, err = ParseJSON(data)
        damn value, err
    } else ready is_json_array(data) {
        sus value JSONValue
        sus err JSONError
        value, err = ParseJSON(data)
        damn value, err
    } else {
        damn JSONValue{type: JSONString, string_value: data}, NoError
    }
}

fr fr Convert JSON value to simple data
slay convert_from_json_value(value JSONValue) tea {
    bestie value.type {
        case JSONString:
            damn value.string_value
        case JSONNumber:
            damn value.number_value
        case JSONBoolean:
            ready value.boolean_value {
                damn "based"
            } otherwise {
                damn "cap"
            }
        case JSONNull:
            damn "cringe"
        case JSONObject:
            damn SerializeJSON(value, cap)
        case JSONArray:
            damn SerializeJSON(value, cap)
        default:
            damn ""
    }
}

fr fr Enhanced numeric validation
slay is_numeric_enhanced(value tea) lit {
    ready value == "" {
        damn cap
    }
    
    sus i drip = 0
    ready value[0] == '-' {
        ready len(value) == 1 {
            damn cap
        }
        i = 1
    }
    
    sus has_dot lit = cap
    sus has_e lit = cap
    
    bestie i < len(value) {
        sus c normie = char_at(value, i)
        ready c >= '0' && c <= '9' {
            fr fr Valid digit
        } else ready c == '.' && !has_dot && !has_e {
            has_dot = based
        } else ready (c == 'e' || c == 'E') && !has_e && i > 0 {
            has_e = based
            ready i + 1 < len(value) && (char_at(value, i + 1) == '+' || char_at(value, i + 1) == '-') {
                i = i + 1
            }
        } else {
            damn cap
        }
        i = i + 1
    }
    
    damn i > 0
}

fr fr Check if string is JSON object
slay is_json_object(value tea) lit {
    sus trimmed tea = trim_whitespace(value)
    ready len(trimmed) < 2 {
        damn cap
    }
    damn char_at(trimmed, 0) == '{' && char_at(trimmed, len(trimmed) - 1) == '}'
}

fr fr Check if string is JSON array
slay is_json_array(value tea) lit {
    sus trimmed tea = trim_whitespace(value)
    ready len(trimmed) < 2 {
        damn cap
    }
    damn char_at(trimmed, 0) == '[' && char_at(trimmed, len(trimmed) - 1) == ']'
}

fr fr Convert error to string
slay error_to_string(err JSONError) tea {
    bestie err {
        case NoError:
            damn "No error"
        case InvalidSyntax:
            damn "Invalid JSON syntax"
        case UnexpectedToken:
            damn "Unexpected token"
        case UnexpectedEnd:
            damn "Unexpected end of input"
        case InvalidEscape:
            damn "Invalid escape sequence"
        case InvalidUnicode:
            damn "Invalid Unicode escape"
        case NumberOutOfRange:
            damn "Number out of range"
        case DepthLimitExceeded:
            damn "Maximum depth exceeded"
        case InvalidUTF8:
            damn "Invalid UTF-8 encoding"
        default:
            damn "Unknown error"
    }
}

fr fr Convert JSON value type to string
slay json_value_type_to_string(t JSONValueType) tea {
    bestie t {
        case JSONString:
            damn "string"
        case JSONNumber:
            damn "number"
        case JSONBoolean:
            damn "boolean"
        case JSONNull:
            damn "null"
        case JSONObject:
            damn "object"
        case JSONArray:
            damn "array"
        default:
            damn "unknown"
    }
}

fr fr Unicode to UTF-8 conversion
slay unicode_to_utf8(codepoint normie) tea {
    ready codepoint < 0x80 {
        damn char_to_string(codepoint)
    } else ready codepoint < 0x800 {
        sus byte1 normie = 0xC0 | (codepoint >> 6)
        sus byte2 normie = 0x80 | (codepoint & 0x3F)
        damn char_to_string(byte1) + char_to_string(byte2)
    } else ready codepoint < 0x10000 {
        sus byte1 normie = 0xE0 | (codepoint >> 12)
        sus byte2 normie = 0x80 | ((codepoint >> 6) & 0x3F)
        sus byte3 normie = 0x80 | (codepoint & 0x3F)
        damn char_to_string(byte1) + char_to_string(byte2) + char_to_string(byte3)
    } else {
        sus byte1 normie = 0xF0 | (codepoint >> 18)
        sus byte2 normie = 0x80 | ((codepoint >> 12) & 0x3F)
        sus byte3 normie = 0x80 | ((codepoint >> 6) & 0x3F)
        sus byte4 normie = 0x80 | (codepoint & 0x3F)
        damn char_to_string(byte1) + char_to_string(byte2) + char_to_string(byte3) + char_to_string(byte4)
    }
}

fr fr Format number as 4-digit hex
slay format_hex_4(n normie) tea {
    sus digits tea = "0123456789ABCDEF"
    damn char_to_string(char_at(digits, (n >> 12) & 0xF)) + 
         char_to_string(char_at(digits, (n >> 8) & 0xF)) + 
         char_to_string(char_at(digits, (n >> 4) & 0xF)) + 
         char_to_string(char_at(digits, n & 0xF))
}

fr fr Trim whitespace
slay trim_whitespace(s tea) tea {
    sus start drip = 0
    sus end drip = len(s)
    
    bestie start < end && is_whitespace_char(char_at(s, start)) {
        start = start + 1
    }
    
    bestie end > start && is_whitespace_char(char_at(s, end - 1)) {
        end = end - 1
    }
    
    damn substring(s, start, end - start)
}

fr fr Check if character is whitespace
slay is_whitespace_char(c normie) lit {
    damn c == ' ' || c == '\t' || c == '\n' || c == '\r'
}

fr fr Add prefix to each line
slay add_prefix_to_lines(text tea, prefix tea) tea {
    ready prefix == "" {
        damn text
    }
    
    sus result tea = prefix
    sus i drip = 0
    bestie i < len(text) {
        sus c normie = char_at(text, i)
        result = result + char_to_string(c)
        ready c == '\n' && i + 1 < len(text) {
            result = result + prefix
        }
        i = i + 1
    }
    
    damn result
}

fr fr =============================================================================
fr fr Legacy Compatibility Functions
fr fr =============================================================================

fr fr Compatibility functions for existing test suite
slay marshal(data tea) tea { damn Marshal(data) }
slay unmarshal(json_string tea) tea { damn Unmarshal(json_string) }
slay parse(json_string tea) tea { damn Unmarshal(json_string) }
slay parse_json(input tea) tea { damn Unmarshal(input) }
slay parse_json_string(json_string tea) tea { damn Unmarshal(json_string) }
slay to_json(data tea) tea { damn Marshal(data) }
slay stringify(data tea) tea { damn Marshal(data) }
slay format_json(data tea) tea { damn MarshalIndent(data, "", "  ") }
slay validate_json(json_string tea) lit { damn IsValidJSON(json_string) }
slay validate_schema(json_string tea, schema tea) lit { damn ValidateSchema(json_string, schema) }

fr fr String operation functions for test compatibility
slay string_length(s tea) drip { damn len(s) }
slay string_contains(haystack tea, needle tea) lit { damn contains(haystack, needle) }
slay string_starts_with(haystack tea, prefix tea) lit { damn starts_with(haystack, prefix) }
slay string_ends_with(haystack tea, suffix tea) lit { damn ends_with(haystack, suffix) }
slay char_at(s tea, i drip) normie { damn s[i] }
slay char_to_string(c normie) tea { damn tea(normie[value]{c}) }
slay substring(s tea, start drip, length drip) tea { damn s[start:start+length] }

fr fr Utility functions
slay contains(haystack tea, needle tea) lit {
    sus haystack_len drip = len(haystack)
    sus needle_len drip = len(needle)
    ready needle_len > haystack_len { damn cap }
    
    sus i drip = 0
    bestie i <= haystack_len - needle_len {
        sus match lit = based
        sus j drip = 0
        bestie j < needle_len {
            ready haystack[i + j] != needle[j] {
                match = cap
                break
            }
            j = j + 1
        }
        ready match { damn based }
        i = i + 1
    }
    damn cap
}

slay starts_with(haystack tea, prefix tea) lit {
    ready len(prefix) > len(haystack) { damn cap }
    damn substring(haystack, 0, len(prefix)) == prefix
}

slay ends_with(haystack tea, suffix tea) lit {
    ready len(suffix) > len(haystack) { damn cap }
    sus start drip = len(haystack) - len(suffix)
    damn substring(haystack, start, len(suffix)) == suffix
}

fr fr Enhanced type checking functions
slay is_object(value tea) lit { damn is_json_object(value) }
slay is_array(value tea) lit { damn is_json_array(value) }
slay is_string(json_value tea) lit { damn get_json_type(json_value) == "string" }
slay is_number(json_value tea) lit { damn get_json_type(json_value) == "number" }
slay is_boolean_value(json_value tea) lit { damn get_json_type(json_value) == "boolean" }
slay is_array_value(json_value tea) lit { damn get_json_type(json_value) == "array" }
slay is_object_value(json_value tea) lit { damn get_json_type(json_value) == "object" }
slay is_null_value(json_value tea) lit { damn get_json_type(json_value) == "null" }
slay is_numeric(value tea) lit { damn is_numeric_enhanced(value) }
slay is_numeric_simple(value tea) lit { damn is_numeric_enhanced(value) }
slay is_valid_json_number(value tea) lit { damn is_numeric_enhanced(value) }
slay is_string_literal(value tea) lit { damn starts_with(value, "\"") && ends_with(value, "\"") }
slay is_boolean(value tea) lit { damn value == "based" || value == "cap" || value == "true" || value == "false" }

fr fr String escaping functions
slay json_escape_string(value tea) tea {
    sus result tea = ""
    sus i drip = 0
    bestie i < len(value) {
        sus c normie = char_at(value, i)
        ready c == '"' {
            result = result + "\\\""
        } else ready c == '\\' {
            result = result + "\\\\"
        } else ready c == '\n' {
            result = result + "\\n"
        } else ready c == '\t' {
            result = result + "\\t"
        } else ready c == '\r' {
            result = result + "\\r"
        } else ready c < 0x20 {
            result = result + "\\u" + format_hex_4(c)
        } else {
            result = result + char_to_string(c)
        }
        i = i + 1
    }
    damn result
}

slay json_unescape_string(value tea) tea {
    sus result tea = ""
    sus i drip = 0
    bestie i < len(value) {
        sus c normie = char_at(value, i)
        ready c == '\\' && i + 1 < len(value) {
            sus next normie = char_at(value, i + 1)
            ready next == '"' {
                result = result + "\""
                i = i + 1
            } else ready next == '\\' {
                result = result + "\\"
                i = i + 1
            } else ready next == 'n' {
                result = result + "\n"
                i = i + 1
            } else ready next == 't' {
                result = result + "\t"
                i = i + 1
            } else ready next == 'r' {
                result = result + "\r"
                i = i + 1
            } else {
                result = result + char_to_string(c)
            }
        } else {
            result = result + char_to_string(c)
        }
        i = i + 1
    }
    damn result
}
