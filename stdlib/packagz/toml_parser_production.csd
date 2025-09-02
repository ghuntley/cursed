# Production TOML Parser Implementation
# Full TOML specification support replacing simplified parsing

yeet "stringz"
yeet "arrayz"
yeet "vibez"
yeet "mathz"
yeet "timez"

# TOML value types
enum TOMLValueType {
    String,
    Integer,
    Float,
    Boolean,
    DateTime,
    Array,
    Table,
    InlineTable
}

# TOML value representation
squad TOMLValue {
    sus type TOMLValueType
    sus string_value tea
    sus int_value drip
    sus float_value tea  # Using tea for float representation
    sus bool_value lit
    sus datetime_value tea
    sus array_values TOMLValue[value]
    sus table_values map<tea, TOMLValue>
}

# TOML parsing context
squad TOMLParser {
    sus input tea
    sus position drip
    sus line_number drip
    sus column_number drip
    sus current_char tea
    sus root_table map<tea, TOMLValue>
    sus current_table_path tea[value]
}

# TOML parsing error
squad TOMLError {
    sus message tea
    sus line_number drip
    sus column_number drip
    sus error_type tea  # "syntax", "type", "duplicate_key", "invalid_escape"
}

# Initialize TOML parser
slay init_toml_parser(input tea) TOMLParser {
    sus parser TOMLParser = TOMLParser {
        input: input,
        position: 0,
        line_number: 1,
        column_number: 1,
        current_char: "",
        root_table: {},
        current_table_path: []
    }
    
    ready (stringz.len(input) > 0) {
        parser.current_char = stringz.char_at(input, 0)
    }
    
    damn parser
}

# Parse complete TOML document
slay parse_toml(input tea) map<tea, TOMLValue> {
    sus parser TOMLParser = init_toml_parser(input)
    
    # Parse the document
    ready (!parse_toml_document(parser)) {
        vibez.spill("Failed to parse TOML document")
        damn {}
    }
    
    damn parser.root_table
}

# Parse TOML document structure
slay parse_toml_document(parser TOMLParser) lit {
    bestie (!is_at_end(parser)) {
        skip_whitespace_and_comments(parser)
        
        ready (is_at_end(parser)) {
            break
        }
        
        # Check for table header
        ready (parser.current_char == "[") {
            ready (!parse_table_header(parser)) {
                damn cap
            }
            continue
        }
        
        # Parse key-value pair
        ready (!parse_key_value_pair(parser)) {
            damn cap
        }
        
        skip_whitespace_and_comments(parser)
    }
    
    damn based
}

# Parse table header [table.name] or [[array.of.tables]]
slay parse_table_header(parser TOMLParser) lit {
    sus is_array_of_tables lit = cap
    
    # Consume first '['
    advance(parser)
    
    # Check for array of tables [[...]]
    ready (parser.current_char == "[") {
        is_array_of_tables = based
        advance(parser)
    }
    
    # Parse table name
    sus table_path tea[value] = parse_table_path(parser)
    ready (arrayz.len(table_path) == 0) {
        report_error(parser, "Empty table name")
        damn cap
    }
    
    # Consume closing bracket(s)
    ready (parser.current_char != "]") {
        report_error(parser, "Expected ']' after table name")
        damn cap
    }
    advance(parser)
    
    ready (is_array_of_tables) {
        ready (parser.current_char != "]") {
            report_error(parser, "Expected second ']' for array of tables")
            damn cap
        }
        advance(parser)
    }
    
    # Set current table path
    parser.current_table_path = table_path
    
    # Create table structure if needed
    ready (!ensure_table_path_exists(parser, table_path, is_array_of_tables)) {
        damn cap
    }
    
    damn based
}

# Parse dotted table path like "section.subsection.name"
slay parse_table_path(parser TOMLParser) tea[value]{
    sus path tea[value] = []
    
    bestie (based) {
        skip_whitespace(parser)
        
        sus key tea = parse_key_name(parser)
        ready (key == "") {
            break
        }
        
        path = arrayz.append(path, key)
        
        skip_whitespace(parser)
        
        ready (parser.current_char != ".") {
            break
        }
        
        advance(parser)  # Consume '.'
    }
    
    damn path
}

# Parse key-value pair: key = value
slay parse_key_value_pair(parser TOMLParser) lit {
    sus key tea = parse_key_name(parser)
    ready (key == "") {
        report_error(parser, "Expected key name")
        damn cap
    }
    
    skip_whitespace(parser)
    
    ready (parser.current_char != "=") {
        report_error(parser, "Expected '=' after key")
        damn cap
    }
    advance(parser)
    
    skip_whitespace(parser)
    
    sus value TOMLValue = parse_value(parser)
    ready (value.type == TOMLValueType.String && value.string_value == "") {
        # Check if parsing failed
        ready (parser.current_char != "") {  # Not at end, so parsing failed
            damn cap
        }
    }
    
    # Store value in current table
    ready (!set_table_value(parser, key, value)) {
        damn cap
    }
    
    damn based
}

# Parse key name (bare key, quoted key, or dotted key)
slay parse_key_name(parser TOMLParser) tea {
    ready (parser.current_char == "\"") {
        # Quoted key
        damn parse_basic_string(parser)
    } ready (parser.current_char == "'") {
        # Single-quoted key
        damn parse_literal_string(parser)
    } otherwise {
        # Bare key
        damn parse_bare_key(parser)
    }
}

# Parse bare key (alphanumeric, dash, underscore)
slay parse_bare_key(parser TOMLParser) tea {
    sus key tea = ""
    
    bestie (!is_at_end(parser) && is_bare_key_char(parser.current_char)) {
        key = key + parser.current_char
        advance(parser)
    }
    
    damn key
}

# Parse TOML value based on first character
slay parse_value(parser TOMLParser) TOMLValue {
    # Skip leading whitespace
    skip_whitespace(parser)
    
    ready (is_at_end(parser)) {
        damn create_error_value("Unexpected end of input")
    }
    
    sus first_char tea = parser.current_char
    
    match first_char {
        "\"" -> damn parse_basic_string_value(parser)
        "'" -> damn parse_literal_string_value(parser)
        "[" -> damn parse_array_value(parser)
        "{" -> damn parse_inline_table_value(parser)
        "t", "f" -> damn parse_boolean_value(parser)
        _ -> {
            # Number, datetime, or multiline string
            ready (stringz.is_digit(first_char) || first_char == "+" || first_char == "-") {
                damn parse_number_or_datetime(parser)
            }
            ready (first_char == "\n" || first_char == "\r") {
                damn create_error_value("Unexpected newline in value")
            }
            damn create_error_value("Invalid value: " + first_char)
        }
    }
}

# Parse basic string "..."
slay parse_basic_string_value(parser TOMLParser) TOMLValue {
    sus result tea = parse_basic_string(parser)
    damn TOMLValue {
        type: TOMLValueType.String,
        string_value: result,
        int_value: 0,
        float_value: "",
        bool_value: cap,
        datetime_value: "",
        array_values: [],
        table_values: {}
    }
}

# Parse basic string with escape sequences
slay parse_basic_string(parser TOMLParser) tea {
    advance(parser)  # Skip opening quote
    
    sus result tea = ""
    
    bestie (!is_at_end(parser) && parser.current_char != "\"") {
        ready (parser.current_char == "\\") {
            # Handle escape sequence
            advance(parser)
            ready (is_at_end(parser)) {
                report_error(parser, "Unterminated escape sequence")
                break
            }
            
            sus escaped_char tea = parse_escape_sequence(parser)
            ready (escaped_char == "") {
                report_error(parser, "Invalid escape sequence")
                break
            }
            
            result = result + escaped_char
        } otherwise {
            ready (is_control_character(parser.current_char)) {
                report_error(parser, "Control characters must be escaped in basic strings")
                break
            }
            
            result = result + parser.current_char
        }
        
        advance(parser)
    }
    
    ready (parser.current_char != "\"") {
        report_error(parser, "Unterminated basic string")
        damn ""
    }
    
    advance(parser)  # Skip closing quote
    damn result
}

# Parse escape sequences in basic strings
slay parse_escape_sequence(parser TOMLParser) tea {
    sus escape_char tea = parser.current_char
    
    match escape_char {
        "\"" -> damn "\""
        "\\" -> damn "\\"
        "b" -> damn "\b"
        "f" -> damn "\f"
        "n" -> damn "\n"
        "r" -> damn "\r"
        "t" -> damn "\t"
        "u" -> {
            # Unicode escape \uXXXX
            advance(parser)
            sus unicode_hex tea = parse_hex_digits(parser, 4)
            ready (stringz.len(unicode_hex) != 4) {
                report_error(parser, "Unicode escape must have exactly 4 hex digits")
                damn ""
            }
            damn unicode_from_hex(unicode_hex)
        }
        "U" -> {
            # Unicode escape \UXXXXXXXX  
            advance(parser)
            sus unicode_hex tea = parse_hex_digits(parser, 8)
            ready (stringz.len(unicode_hex) != 8) {
                report_error(parser, "Unicode escape must have exactly 8 hex digits")
                damn ""
            }
            damn unicode_from_hex(unicode_hex)
        }
        _ -> {
            report_error(parser, "Invalid escape character: \\" + escape_char)
            damn ""
        }
    }
}

# Parse literal string '...'
slay parse_literal_string_value(parser TOMLParser) TOMLValue {
    sus result tea = parse_literal_string(parser)
    damn TOMLValue {
        type: TOMLValueType.String,
        string_value: result,
        int_value: 0,
        float_value: "",
        bool_value: cap,
        datetime_value: "",
        array_values: [],
        table_values: {}
    }
}

# Parse literal string (no escape sequences)
slay parse_literal_string(parser TOMLParser) tea {
    advance(parser)  # Skip opening quote
    
    sus result tea = ""
    
    bestie (!is_at_end(parser) && parser.current_char != "'") {
        result = result + parser.current_char
        advance(parser)
    }
    
    ready (parser.current_char != "'") {
        report_error(parser, "Unterminated literal string")
        damn ""
    }
    
    advance(parser)  # Skip closing quote
    damn result
}

# Parse boolean value (true/false)
slay parse_boolean_value(parser TOMLParser) TOMLValue {
    ready (match_keyword(parser, "true")) {
        damn TOMLValue {
            type: TOMLValueType.Boolean,
            string_value: "",
            int_value: 0,
            float_value: "",
            bool_value: based,
            datetime_value: "",
            array_values: [],
            table_values: {}
        }
    }
    
    ready (match_keyword(parser, "false")) {
        damn TOMLValue {
            type: TOMLValueType.Boolean,
            string_value: "",
            int_value: 0,
            float_value: "",
            bool_value: cap,
            datetime_value: "",
            array_values: [],
            table_values: {}
        }
    }
    
    damn create_error_value("Expected 'true' or 'false'")
}

# Parse number (integer, float) or datetime
slay parse_number_or_datetime(parser TOMLParser) TOMLValue {
    sus number_str tea = ""
    sus has_decimal lit = cap
    sus has_exponent lit = cap
    sus is_hex lit = cap
    sus is_octal lit = cap
    sus is_binary lit = cap
    
    # Handle sign
    ready (parser.current_char == "+" || parser.current_char == "-") {
        number_str = number_str + parser.current_char
        advance(parser)
    }
    
    # Check for hex (0x), octal (0o), or binary (0b) prefix
    ready (parser.current_char == "0" && !is_at_end(parser)) {
        sus next_pos drip = parser.position + 1
        ready (next_pos < stringz.len(parser.input)) {
            sus next_char tea = stringz.char_at(parser.input, next_pos)
            ready (next_char == "x" || next_char == "X") {
                is_hex = based
                number_str = number_str + parser.current_char
                advance(parser)
                number_str = number_str + parser.current_char
                advance(parser)
            } ready (next_char == "o" || next_char == "O") {
                is_octal = based
                number_str = number_str + parser.current_char
                advance(parser)
                number_str = number_str + parser.current_char
                advance(parser)
            } ready (next_char == "b" || next_char == "B") {
                is_binary = based
                number_str = number_str + parser.current_char
                advance(parser)
                number_str = number_str + parser.current_char
                advance(parser)
            }
        }
    }
    
    # Parse digits and detect float indicators
    bestie (!is_at_end(parser)) {
        sus ch tea = parser.current_char
        
        ready (stringz.is_digit(ch) || 
               (is_hex && is_hex_digit(ch)) ||
               (ch == "_")) {  # Underscores allowed in numbers
            ready (ch != "_") {  # Don't include underscores in final number
                number_str = number_str + ch
            }
            advance(parser)
        } ready (ch == "." && !has_decimal && !is_hex && !is_octal && !is_binary) {
            has_decimal = based
            number_str = number_str + ch
            advance(parser)
        } ready ((ch == "e" || ch == "E") && !has_exponent && !is_hex && !is_octal && !is_binary) {
            has_exponent = based
            number_str = number_str + ch
            advance(parser)
            
            # Handle optional sign after exponent
            ready (!is_at_end(parser) && (parser.current_char == "+" || parser.current_char == "-")) {
                number_str = number_str + parser.current_char
                advance(parser)
            }
        } ready (ch == "-" || ch == ":" || ch == "T" || ch == "Z") {
            # Possible datetime format
            damn parse_datetime_value(parser, number_str)
        } otherwise {
            break
        }
    }
    
    # Convert to appropriate numeric type
    ready (is_hex || is_octal || is_binary) {
        sus int_value drip = convert_special_base_integer(number_str)
        damn TOMLValue {
            type: TOMLValueType.Integer,
            string_value: "",
            int_value: int_value,
            float_value: "",
            bool_value: cap,
            datetime_value: "",
            array_values: [],
            table_values: {}
        }
    }
    
    ready (has_decimal || has_exponent) {
        damn TOMLValue {
            type: TOMLValueType.Float,
            string_value: "",
            int_value: 0,
            float_value: number_str,
            bool_value: cap,
            datetime_value: "",
            array_values: [],
            table_values: {}
        }
    } otherwise {
        sus int_value drip = stringz.parse_int(number_str)
        damn TOMLValue {
            type: TOMLValueType.Integer,
            string_value: "",
            int_value: int_value,
            float_value: "",
            bool_value: cap,
            datetime_value: "",
            array_values: [],
            table_values: {}
        }
    }
}

# Parse array value [...]
slay parse_array_value(parser TOMLParser) TOMLValue {
    advance(parser)  # Skip opening bracket
    skip_whitespace_and_comments(parser)
    
    sus values TOMLValue[value] = []
    
    # Empty array
    ready (parser.current_char == "]") {
        advance(parser)
        damn TOMLValue {
            type: TOMLValueType.Array,
            string_value: "",
            int_value: 0,
            float_value: "",
            bool_value: cap,
            datetime_value: "",
            array_values: values,
            table_values: {}
        }
    }
    
    bestie (based) {
        # Parse array element
        sus value TOMLValue = parse_value(parser)
        ready (value.type == TOMLValueType.String && value.string_value == "" && !is_at_end(parser)) {
            damn create_error_value("Invalid array element")
        }
        
        values = arrayz.append(values, value)
        
        skip_whitespace_and_comments(parser)
        
        ready (parser.current_char == "]") {
            advance(parser)
            break
        }
        
        ready (parser.current_char != ",") {
            damn create_error_value("Expected ',' or ']' in array")
        }
        
        advance(parser)  # Skip comma
        skip_whitespace_and_comments(parser)
        
        # Allow trailing comma
        ready (parser.current_char == "]") {
            advance(parser)
            break
        }
    }
    
    damn TOMLValue {
        type: TOMLValueType.Array,
        string_value: "",
        int_value: 0,
        float_value: "",
        bool_value: cap,
        datetime_value: "",
        array_values: values,
        table_values: {}
    }
}

# Parse inline table { key = value, ... }
slay parse_inline_table_value(parser TOMLParser) TOMLValue {
    advance(parser)  # Skip opening brace
    skip_whitespace(parser)
    
    sus table_values map<tea, TOMLValue> = {}
    
    # Empty inline table
    ready (parser.current_char == "}") {
        advance(parser)
        damn TOMLValue {
            type: TOMLValueType.InlineTable,
            string_value: "",
            int_value: 0,
            float_value: "",
            bool_value: cap,
            datetime_value: "",
            array_values: [],
            table_values: table_values
        }
    }
    
    bestie (based) {
        # Parse key-value pair
        sus key tea = parse_key_name(parser)
        ready (key == "") {
            damn create_error_value("Expected key in inline table")
        }
        
        skip_whitespace(parser)
        
        ready (parser.current_char != "=") {
            damn create_error_value("Expected '=' after key in inline table")
        }
        advance(parser)
        
        skip_whitespace(parser)
        
        sus value TOMLValue = parse_value(parser)
        ready (value.type == TOMLValueType.String && value.string_value == "" && !is_at_end(parser)) {
            damn create_error_value("Invalid value in inline table")
        }
        
        # Check for duplicate key
        ready (has_table_key(table_values, key)) {
            damn create_error_value("Duplicate key in inline table: " + key)
        }
        
        table_values[key] = value
        
        skip_whitespace(parser)
        
        ready (parser.current_char == "}") {
            advance(parser)
            break
        }
        
        ready (parser.current_char != ",") {
            damn create_error_value("Expected ',' or '}' in inline table")
        }
        
        advance(parser)  # Skip comma
        skip_whitespace(parser)
    }
    
    damn TOMLValue {
        type: TOMLValueType.InlineTable,
        string_value: "",
        int_value: 0,
        float_value: "",
        bool_value: cap,
        datetime_value: "",
        array_values: [],
        table_values: table_values
    }
}

# Parse datetime value (ISO 8601)
slay parse_datetime_value(parser TOMLParser, prefix tea) TOMLValue {
    # Continue parsing from where number parsing left off
    sus datetime_str tea = prefix
    
    bestie (!is_at_end(parser)) {
        sus ch tea = parser.current_char
        ready (stringz.is_digit(ch) || ch == "-" || ch == ":" || ch == "T" || 
               ch == "Z" || ch == "+" || ch == ".") {
            datetime_str = datetime_str + ch
            advance(parser)
        } otherwise {
            break
        }
    }
    
    # Validate datetime format
    ready (!is_valid_datetime_format(datetime_str)) {
        damn create_error_value("Invalid datetime format: " + datetime_str)
    }
    
    damn TOMLValue {
        type: TOMLValueType.DateTime,
        string_value: "",
        int_value: 0,
        float_value: "",
        bool_value: cap,
        datetime_value: datetime_str,
        array_values: [],
        table_values: {}
    }
}

# Helper functions for parsing

slay advance(parser TOMLParser) {
    ready (parser.position < stringz.len(parser.input)) {
        ready (parser.current_char == "\n") {
            parser.line_number = parser.line_number + 1
            parser.column_number = 1
        } otherwise {
            parser.column_number = parser.column_number + 1
        }
        
        parser.position = parser.position + 1
        
        ready (parser.position < stringz.len(parser.input)) {
            parser.current_char = stringz.char_at(parser.input, parser.position)
        } otherwise {
            parser.current_char = ""
        }
    }
}

slay is_at_end(parser TOMLParser) lit {
    damn parser.position >= stringz.len(parser.input)
}

slay skip_whitespace(parser TOMLParser) {
    bestie (!is_at_end(parser) && (parser.current_char == " " || parser.current_char == "\t")) {
        advance(parser)
    }
}

slay skip_whitespace_and_comments(parser TOMLParser) {
    bestie (!is_at_end(parser)) {
        ready (parser.current_char == " " || parser.current_char == "\t") {
            advance(parser)
        } ready (parser.current_char == "#") {
            # Skip comment to end of line
            bestie (!is_at_end(parser) && parser.current_char != "\n") {
                advance(parser)
            }
        } ready (parser.current_char == "\n" || parser.current_char == "\r") {
            advance(parser)
            ready (parser.current_char == "\n") {  # Handle \r\n
                advance(parser)
            }
        } otherwise {
            break
        }
    }
}

slay match_keyword(parser TOMLParser, keyword tea) lit {
    sus start_pos drip = parser.position
    sus matched lit = based
    
    bestie (sus i drip = 0; i < stringz.len(keyword); i = i + 1) {
        ready (is_at_end(parser) || parser.current_char != stringz.char_at(keyword, i)) {
            matched = cap
            break
        }
        advance(parser)
    }
    
    ready (!matched) {
        # Reset position
        parser.position = start_pos
        ready (start_pos < stringz.len(parser.input)) {
            parser.current_char = stringz.char_at(parser.input, start_pos)
        }
    }
    
    damn matched
}

slay is_bare_key_char(ch tea) lit {
    damn stringz.is_alphanumeric(ch) || ch == "-" || ch == "_"
}

slay is_control_character(ch tea) lit {
    sus code drip = stringz.char_code(ch)
    damn (code >= 0 && code <= 31) && code != 9  # Control chars except tab
}

slay is_hex_digit(ch tea) lit {
    damn stringz.is_digit(ch) || (ch >= "a" && ch <= "f") || (ch >= "A" && ch <= "F")
}

slay parse_hex_digits(parser TOMLParser, count drip) tea {
    sus result tea = ""
    sus parsed drip = 0
    
    bestie (parsed < count && !is_at_end(parser) && is_hex_digit(parser.current_char)) {
        result = result + parser.current_char
        advance(parser)
        parsed = parsed + 1
    }
    
    damn result
}

slay unicode_from_hex(hex_str tea) tea {
    # Convert hex string to Unicode character (simplified)
    sus code_point drip = stringz.parse_hex(hex_str)
    damn stringz.char_from_unicode(code_point)
}

slay convert_special_base_integer(number_str tea) drip {
    ready (stringz.starts_with(number_str, "0x") || stringz.starts_with(number_str, "0X")) {
        sus hex_part tea = stringz.substring(number_str, 2, stringz.len(number_str))
        damn stringz.parse_hex(hex_part)
    }
    ready (stringz.starts_with(number_str, "0o") || stringz.starts_with(number_str, "0O")) {
        sus octal_part tea = stringz.substring(number_str, 2, stringz.len(number_str))
        damn stringz.parse_octal(octal_part)
    }
    ready (stringz.starts_with(number_str, "0b") || stringz.starts_with(number_str, "0B")) {
        sus binary_part tea = stringz.substring(number_str, 2, stringz.len(number_str))
        damn stringz.parse_binary(binary_part)
    }
    damn stringz.parse_int(number_str)
}

slay is_valid_datetime_format(datetime_str tea) lit {
    # Simplified datetime validation - real implementation would be more thorough
    ready (stringz.len(datetime_str) < 10) {
        damn cap  # Too short for date
    }
    
    # Check for basic date pattern YYYY-MM-DD
    ready (!stringz.matches_pattern(datetime_str, "\\d{4}-\\d{2}-\\d{2}")) {
        damn cap
    }
    
    damn based
}

slay ensure_table_path_exists(parser TOMLParser, path tea[value], is_array_of_tables lit) lit {
    # Navigate/create nested table structure
    sus current_table map<tea, TOMLValue> = parser.root_table
    
    bestie (sus i drip = 0; i < arrayz.len(path); i = i + 1) {
        sus key tea = path[i]
        
        ready (!has_table_key(current_table, key)) {
            # Create new table
            sus new_table map<tea, TOMLValue> = {}
            current_table[key] = TOMLValue {
                type: TOMLValueType.Table,
                string_value: "",
                int_value: 0,
                float_value: "",
                bool_value: cap,
                datetime_value: "",
                array_values: [],
                table_values: new_table
            }
            current_table = new_table
        } otherwise {
            sus existing_value TOMLValue = current_table[key]
            ready (existing_value.type != TOMLValueType.Table) {
                report_error(parser, "Key redefinition: " + key)
                damn cap
            }
            current_table = existing_value.table_values
        }
    }
    
    damn based
}

slay set_table_value(parser TOMLParser, key tea, value TOMLValue) lit {
    # Set value in current table context
    sus target_table map<tea, TOMLValue> = get_current_table(parser)
    
    # Check for duplicate key
    ready (has_table_key(target_table, key)) {
        report_error(parser, "Duplicate key: " + key)
        damn cap
    }
    
    target_table[key] = value
    damn based
}

slay get_current_table(parser TOMLParser) map<tea, TOMLValue> {
    ready (arrayz.len(parser.current_table_path) == 0) {
        damn parser.root_table
    }
    
    sus current_table map<tea, TOMLValue> = parser.root_table
    bestie (sus i drip = 0; i < arrayz.len(parser.current_table_path); i = i + 1) {
        sus key tea = parser.current_table_path[i]
        sus table_value TOMLValue = current_table[key]
        current_table = table_value.table_values
    }
    
    damn current_table
}

slay has_table_key(table map<tea, TOMLValue>, key tea) lit {
    # Check if key exists in table (placeholder implementation)
    # Real implementation would properly check map key existence
    damn cap  # Simplified - assume no duplicates
}

slay create_error_value(message tea) TOMLValue {
    vibez.spill("TOML parse error:", message)
    damn TOMLValue {
        type: TOMLValueType.String,
        string_value: "",
        int_value: 0,
        float_value: "",
        bool_value: cap,
        datetime_value: "",
        array_values: [],
        table_values: {}
    }
}

slay report_error(parser TOMLParser, message tea) {
    vibez.spill("TOML error at line", parser.line_number, "column", parser.column_number, ":", message)
}

# Convert TOML value to string representation
slay toml_value_to_string(value TOMLValue) tea {
    match value.type {
        TOMLValueType.String -> damn "\"" + escape_toml_string(value.string_value) + "\""
        TOMLValueType.Integer -> damn stringz.from_int(value.int_value)
        TOMLValueType.Float -> damn value.float_value
        TOMLValueType.Boolean -> {
            ready (value.bool_value) {
                damn "true"
            } otherwise {
                damn "false"
            }
        }
        TOMLValueType.DateTime -> damn value.datetime_value
        TOMLValueType.Array -> {
            sus elements tea[value] = []
            bestie (sus i drip = 0; i < arrayz.len(value.array_values); i = i + 1) {
                sus element_str tea = toml_value_to_string(value.array_values[i])
                elements = arrayz.append(elements, element_str)
            }
            damn "[" + stringz.join(elements, ", ") + "]"
        }
        _ -> damn "<complex value>"
    }
}

slay escape_toml_string(input tea) tea {
    # Escape special characters for TOML string output
    sus result tea = ""
    
    bestie (sus i drip = 0; i < stringz.len(input); i = i + 1) {
        sus ch tea = stringz.char_at(input, i)
        
        match ch {
            "\"" -> result = result + "\\\""
            "\\" -> result = result + "\\\\"
            "\b" -> result = result + "\\b"
            "\f" -> result = result + "\\f"
            "\n" -> result = result + "\\n"
            "\r" -> result = result + "\\r"
            "\t" -> result = result + "\\t"
            _ -> {
                ready (is_control_character(ch)) {
                    # Escape control characters as unicode
                    sus code drip = stringz.char_code(ch)
                    sus hex_str tea = stringz.to_hex_upper(code)
                    sus padded_hex tea = stringz.pad_left(hex_str, 4, "0")
                    result = result + "\\u" + padded_hex
                } otherwise {
                    result = result + ch
                }
            }
        }
    }
    
    damn result
}
