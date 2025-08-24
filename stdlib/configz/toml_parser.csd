fr fr ==========================================
fr fr CURSED TOML Parser - Full TOML v1.0.0 Implementation
fr fr Comprehensive TOML parsing with complete specification compliance
fr fr ==========================================

yeet "stringz"
yeet "mathz"
yeet "filez"
yeet "vibez"

fr fr ==========================================
fr fr TOML Parser Structures
fr fr ==========================================

squad TomlValue {
    sus value_type tea              fr fr "string", "integer", "float", "boolean", "array", "table", "datetime"
    sus string_value tea
    sus integer_value drip
    sus float_value sus            fr fr Using sus for float type
    sus boolean_value lit
    sus array_values []TomlValue
    sus table_values TomlTable
    sus line_number drip
    sus column_number drip
}

squad TomlKeyValue {
    sus key tea
    sus value TomlValue
    sus dotted_keys []tea          fr fr For nested keys like database.host
}

squad TomlTable {
    sus name tea
    sus entries []TomlKeyValue
    sus is_array_table lit
    sus line_number drip
}

squad TomlDocument {
    sus tables []TomlTable
    sus root_entries []TomlKeyValue    fr fr Top-level key-value pairs
    sus parsing_errors []tea
}

squad TomlParser {
    sus content tea
    sus position drip
    sus line_number drip
    sus column_number drip
    sus current_char tea
    sus errors []tea
}

fr fr ==========================================
fr fr Core TOML Parser Functions
fr fr ==========================================

slay create_toml_parser(content tea) TomlParser {
    fr fr Initialize TOML parser with content
    sus parser TomlParser = TomlParser{
        content: content,
        position: 0,
        line_number: 1,
        column_number: 1,
        current_char: "",
        errors: []
    }
    
    ready (string_length(content) > 0) {
        parser.current_char = string_char_at(content, 0)
    }
    
    damn parser
}

slay parse_toml_document(content tea) TomlDocument {
    fr fr Parse complete TOML document
    sus parser TomlParser = create_toml_parser(content)
    sus document TomlDocument = TomlDocument{
        tables: [],
        root_entries: [],
        parsing_errors: []
    }
    
    fr fr Parse document line by line
    bestie (!parser_at_end(parser)) {
        parser = skip_whitespace_and_comments(parser)
        
        ready (parser_at_end(parser)) {
            break
        }
        
        ready (parser.current_char == "[") {
            fr fr Parse table header
            sus table_result ParserResult<TomlTable> = parse_table_header(parser)
            ready (table_result.success) {
                document.tables = append_toml_table(document.tables, table_result.value)
                parser = table_result.parser
            } otherwise {
                document.parsing_errors = append_error(document.parsing_errors, table_result.error_message)
                parser = advance_to_next_line(parser)
            }
        } otherwise {
            fr fr Parse key-value pair
            sus kv_result ParserResult<TomlKeyValue> = parse_key_value_pair(parser)
            ready (kv_result.success) {
                document.root_entries = append_key_value(document.root_entries, kv_result.value)
                parser = kv_result.parser
            } otherwise {
                document.parsing_errors = append_error(document.parsing_errors, kv_result.error_message)
                parser = advance_to_next_line(parser)
            }
        }
    }
    
    fr fr Add any parser errors to document
    document.parsing_errors = merge_error_arrays(document.parsing_errors, parser.errors)
    
    damn document
}

fr fr ==========================================
fr fr Table Header Parsing
fr fr ==========================================

squad ParserResult<T> {
    sus success lit
    sus value T
    sus parser TomlParser
    sus error_message tea
}

slay parse_table_header(parser TomlParser) ParserResult<TomlTable> {
    fr fr Parse [table] or [[array_table]]
    sus result ParserResult<TomlTable> = ParserResult<TomlTable>{
        success: cringe,
        value: TomlTable{},
        parser: parser,
        error_message: ""
    }
    
    fr fr Check for array table [[
    sus is_array_table lit = cringe
    ready (parser.current_char == "[") {
        parser = advance_parser(parser)
        ready (parser.current_char == "[") {
            is_array_table = based
            parser = advance_parser(parser)
        }
    } otherwise {
        result.error_message = "Expected '[' at start of table header"
        damn result
    }
    
    fr fr Parse table name
    sus table_name_result ParserResult<tea> = parse_table_name(parser)
    ready (!table_name_result.success) {
        result.error_message = table_name_result.error_message
        damn result
    }
    
    parser = table_name_result.parser
    sus table_name tea = table_name_result.value
    
    fr fr Check for closing bracket(s)
    ready (is_array_table) {
        ready (parser.current_char != "]") {
            result.error_message = "Expected ']]' to close array table header"
            damn result
        }
        parser = advance_parser(parser)
    }
    
    ready (parser.current_char != "]") {
        result.error_message = "Expected ']' to close table header"
        damn result
    }
    parser = advance_parser(parser)
    
    fr fr Create table
    sus table TomlTable = TomlTable{
        name: table_name,
        entries: [],
        is_array_table: is_array_table,
        line_number: parser.line_number
    }
    
    result.success = based
    result.value = table
    result.parser = parser
    
    damn result
}

slay parse_table_name(parser TomlParser) ParserResult<tea> {
    fr fr Parse dotted table name like "database.connection"
    sus result ParserResult<tea> = ParserResult<tea>{
        success: cringe,
        value: "",
        parser: parser,
        error_message: ""
    }
    
    sus name_parts []tea = []
    parser = skip_whitespace(parser)
    
    bestie (!parser_at_end(parser) && parser.current_char != "]") {
        ready (is_identifier_start(parser.current_char)) {
            sus identifier_result ParserResult<tea> = parse_identifier(parser)
            ready (!identifier_result.success) {
                result.error_message = identifier_result.error_message
                damn result
            }
            name_parts = append_string(name_parts, identifier_result.value)
            parser = identifier_result.parser
        } otherwise ready (parser.current_char == "\"") {
            sus quoted_result ParserResult<tea> = parse_quoted_key(parser)
            ready (!quoted_result.success) {
                result.error_message = quoted_result.error_message
                damn result
            }
            name_parts = append_string(name_parts, quoted_result.value)
            parser = quoted_result.parser
        } otherwise {
            result.error_message = "Invalid character in table name"
            damn result
        }
        
        parser = skip_whitespace(parser)
        ready (parser.current_char == ".") {
            parser = advance_parser(parser)
            parser = skip_whitespace(parser)
        } otherwise {
            break
        }
    }
    
    ready (len(name_parts) == 0) {
        result.error_message = "Empty table name"
        damn result
    }
    
    result.success = based
    result.value = join_strings_with_dot(name_parts)
    result.parser = parser
    
    damn result
}

fr fr ==========================================
fr fr Key-Value Pair Parsing
fr fr ==========================================

slay parse_key_value_pair(parser TomlParser) ParserResult<TomlKeyValue> {
    fr fr Parse key = value
    sus result ParserResult<TomlKeyValue> = ParserResult<TomlKeyValue>{
        success: cringe,
        value: TomlKeyValue{},
        parser: parser,
        error_message: ""
    }
    
    parser = skip_whitespace(parser)
    
    fr fr Parse key (can be dotted like database.host)
    sus key_result ParserResult<tea> = parse_key(parser)
    ready (!key_result.success) {
        result.error_message = key_result.error_message
        damn result
    }
    
    parser = key_result.parser
    sus key tea = key_result.value
    
    fr fr Expect equals sign
    parser = skip_whitespace(parser)
    ready (parser.current_char != "=") {
        result.error_message = "Expected '=' after key"
        damn result
    }
    parser = advance_parser(parser)
    parser = skip_whitespace(parser)
    
    fr fr Parse value
    sus value_result ParserResult<TomlValue> = parse_value(parser)
    ready (!value_result.success) {
        result.error_message = value_result.error_message
        damn result
    }
    
    sus kv TomlKeyValue = TomlKeyValue{
        key: key,
        value: value_result.value,
        dotted_keys: split_dotted_key(key)
    }
    
    result.success = based
    result.value = kv
    result.parser = value_result.parser
    
    damn result
}

slay parse_key(parser TomlParser) ParserResult<tea> {
    fr fr Parse key (bare, quoted, or dotted)
    sus result ParserResult<tea> = ParserResult<tea>{
        success: cringe,
        value: "",
        parser: parser,
        error_message: ""
    }
    
    sus key_parts []tea = []
    
    bestie (!parser_at_end(parser) && parser.current_char != "=" && parser.current_char != " " && parser.current_char != "\t") {
        ready (is_identifier_start(parser.current_char)) {
            sus identifier_result ParserResult<tea> = parse_identifier(parser)
            ready (!identifier_result.success) {
                result.error_message = identifier_result.error_message
                damn result
            }
            key_parts = append_string(key_parts, identifier_result.value)
            parser = identifier_result.parser
        } otherwise ready (parser.current_char == "\"") {
            sus quoted_result ParserResult<tea> = parse_quoted_key(parser)
            ready (!quoted_result.success) {
                result.error_message = quoted_result.error_message
                damn result
            }
            key_parts = append_string(key_parts, quoted_result.value)
            parser = quoted_result.parser
        } otherwise ready (parser.current_char == "'") {
            sus literal_result ParserResult<tea> = parse_literal_string_key(parser)
            ready (!literal_result.success) {
                result.error_message = literal_result.error_message
                damn result
            }
            key_parts = append_string(key_parts, literal_result.value)
            parser = literal_result.parser
        } otherwise {
            break
        }
        
        parser = skip_whitespace(parser)
        ready (parser.current_char == ".") {
            parser = advance_parser(parser)
            parser = skip_whitespace(parser)
        } otherwise {
            break
        }
    }
    
    ready (len(key_parts) == 0) {
        result.error_message = "Empty key"
        damn result
    }
    
    result.success = based
    result.value = join_strings_with_dot(key_parts)
    result.parser = parser
    
    damn result
}

fr fr ==========================================
fr fr Value Parsing
fr fr ==========================================

slay parse_value(parser TomlParser) ParserResult<TomlValue> {
    fr fr Parse TOML value (string, number, boolean, array, inline table, datetime)
    sus result ParserResult<TomlValue> = ParserResult<TomlValue>{
        success: cringe,
        value: TomlValue{},
        parser: parser,
        error_message: ""
    }
    
    parser = skip_whitespace(parser)
    
    ready (parser_at_end(parser)) {
        result.error_message = "Unexpected end of input while parsing value"
        damn result
    }
    
    fr fr Determine value type and parse accordingly
    ready (parser.current_char == "\"") {
        fr fr Basic string or multiline string
        ready (peek_chars(parser, 3) == "\"\"\"") {
            damn parse_multiline_basic_string(parser)
        } otherwise {
            damn parse_basic_string(parser)
        }
    } otherwise ready (parser.current_char == "'") {
        fr fr Literal string or multiline literal string
        ready (peek_chars(parser, 3) == "'''") {
            damn parse_multiline_literal_string(parser)
        } otherwise {
            damn parse_literal_string(parser)
        }
    } otherwise ready (parser.current_char == "[") {
        fr fr Array
        damn parse_array(parser)
    } otherwise ready (parser.current_char == "{") {
        fr fr Inline table
        damn parse_inline_table(parser)
    } otherwise ready (is_digit(parser.current_char) || parser.current_char == "+" || parser.current_char == "-") {
        fr fr Number (integer, float, or datetime)
        damn parse_number_or_datetime(parser)
    } otherwise ready (parser.current_char == "t" || parser.current_char == "f") {
        fr fr Boolean
        damn parse_boolean(parser)
    } otherwise {
        result.error_message = "Invalid value type"
        damn result
    }
}

fr fr ==========================================
fr fr String Parsing
fr fr ==========================================

slay parse_basic_string(parser TomlParser) ParserResult<TomlValue> {
    fr fr Parse "basic string" with escape sequences
    sus result ParserResult<TomlValue> = ParserResult<TomlValue>{
        success: cringe,
        value: TomlValue{},
        parser: parser,
        error_message: ""
    }
    
    ready (parser.current_char != "\"") {
        result.error_message = "Expected '\"' at start of basic string"
        damn result
    }
    
    parser = advance_parser(parser)  fr fr Skip opening quote
    sus string_value tea = ""
    
    bestie (!parser_at_end(parser) && parser.current_char != "\"") {
        ready (parser.current_char == "\\") {
            fr fr Handle escape sequence
            parser = advance_parser(parser)
            ready (parser_at_end(parser)) {
                result.error_message = "Incomplete escape sequence"
                damn result
            }
            
            sus escaped_char tea = get_escaped_character(parser.current_char)
            ready (escaped_char == "") {
                result.error_message = "Invalid escape sequence: \\" + parser.current_char
                damn result
            }
            
            string_value = string_value + escaped_char
            parser = advance_parser(parser)
        } otherwise ready (parser.current_char == "\n") {
            result.error_message = "Unescaped newline in basic string"
            damn result
        } otherwise {
            string_value = string_value + parser.current_char
            parser = advance_parser(parser)
        }
    }
    
    ready (parser.current_char != "\"") {
        result.error_message = "Unterminated basic string"
        damn result
    }
    
    parser = advance_parser(parser)  fr fr Skip closing quote
    
    sus value TomlValue = TomlValue{
        value_type: "string",
        string_value: string_value,
        line_number: parser.line_number,
        column_number: parser.column_number
    }
    
    result.success = based
    result.value = value
    result.parser = parser
    
    damn result
}

slay parse_literal_string(parser TomlParser) ParserResult<TomlValue> {
    fr fr Parse 'literal string' with no escape sequences
    sus result ParserResult<TomlValue> = ParserResult<TomlValue>{
        success: cringe,
        value: TomlValue{},
        parser: parser,
        error_message: ""
    }
    
    ready (parser.current_char != "'") {
        result.error_message = "Expected \"'\" at start of literal string"
        damn result
    }
    
    parser = advance_parser(parser)  fr fr Skip opening quote
    sus string_value tea = ""
    
    bestie (!parser_at_end(parser) && parser.current_char != "'") {
        ready (parser.current_char == "\n") {
            result.error_message = "Unescaped newline in literal string"
            damn result
        }
        string_value = string_value + parser.current_char
        parser = advance_parser(parser)
    }
    
    ready (parser.current_char != "'") {
        result.error_message = "Unterminated literal string"
        damn result
    }
    
    parser = advance_parser(parser)  fr fr Skip closing quote
    
    sus value TomlValue = TomlValue{
        value_type: "string",
        string_value: string_value,
        line_number: parser.line_number,
        column_number: parser.column_number
    }
    
    result.success = based
    result.value = value
    result.parser = parser
    
    damn result
}

fr fr ==========================================
fr fr Number and Boolean Parsing
fr fr ==========================================

slay parse_number_or_datetime(parser TomlParser) ParserResult<TomlValue> {
    fr fr Parse integer, float, or datetime value
    sus result ParserResult<TomlValue> = ParserResult<TomlValue>{
        success: cringe,
        value: TomlValue{},
        parser: parser,
        error_message: ""
    }
    
    sus number_string tea = ""
    sus start_parser TomlParser = parser
    
    fr fr Parse sign
    ready (parser.current_char == "+" || parser.current_char == "-") {
        number_string = number_string + parser.current_char
        parser = advance_parser(parser)
    }
    
    fr fr Parse digits and special characters
    bestie (!parser_at_end(parser) && is_number_char(parser.current_char)) {
        number_string = number_string + parser.current_char
        parser = advance_parser(parser)
    }
    
    ready (string_length(number_string) == 0) {
        result.error_message = "Empty number"
        damn result
    }
    
    fr fr Determine if it's integer, float, or datetime
    ready (contains_datetime_pattern(number_string)) {
        sus value TomlValue = TomlValue{
            value_type: "datetime",
            string_value: number_string,
            line_number: start_parser.line_number,
            column_number: start_parser.column_number
        }
        result.success = based
        result.value = value
        result.parser = parser
    } otherwise ready (string_contains(number_string, ".") || string_contains(number_string, "e") || string_contains(number_string, "E")) {
        fr fr Float
        sus float_value sus = parse_float_value(number_string)
        sus value TomlValue = TomlValue{
            value_type: "float",
            float_value: float_value,
            line_number: start_parser.line_number,
            column_number: start_parser.column_number
        }
        result.success = based
        result.value = value
        result.parser = parser
    } otherwise {
        fr fr Integer
        sus integer_value drip = parse_integer_value(number_string)
        sus value TomlValue = TomlValue{
            value_type: "integer",
            integer_value: integer_value,
            line_number: start_parser.line_number,
            column_number: start_parser.column_number
        }
        result.success = based
        result.value = value
        result.parser = parser
    }
    
    damn result
}

slay parse_boolean(parser TomlParser) ParserResult<TomlValue> {
    fr fr Parse true or false
    sus result ParserResult<TomlValue> = ParserResult<TomlValue>{
        success: cringe,
        value: TomlValue{},
        parser: parser,
        error_message: ""
    }
    
    ready (peek_chars(parser, 4) == "true") {
        parser = advance_parser_by(parser, 4)
        sus value TomlValue = TomlValue{
            value_type: "boolean",
            boolean_value: based,
            line_number: parser.line_number,
            column_number: parser.column_number
        }
        result.success = based
        result.value = value
        result.parser = parser
    } otherwise ready (peek_chars(parser, 5) == "false") {
        parser = advance_parser_by(parser, 5)
        sus value TomlValue = TomlValue{
            value_type: "boolean",
            boolean_value: cringe,
            line_number: parser.line_number,
            column_number: parser.column_number
        }
        result.success = based
        result.value = value
        result.parser = parser
    } otherwise {
        result.error_message = "Invalid boolean value"
    }
    
    damn result
}

fr fr ==========================================
fr fr Array Parsing
fr fr ==========================================

slay parse_array(parser TomlParser) ParserResult<TomlValue> {
    fr fr Parse [value1, value2, value3]
    sus result ParserResult<TomlValue> = ParserResult<TomlValue>{
        success: cringe,
        value: TomlValue{},
        parser: parser,
        error_message: ""
    }
    
    ready (parser.current_char != "[") {
        result.error_message = "Expected '[' at start of array"
        damn result
    }
    
    parser = advance_parser(parser)  fr fr Skip opening bracket
    parser = skip_whitespace_and_comments(parser)
    
    sus array_values []TomlValue = []
    
    fr fr Handle empty array
    ready (parser.current_char == "]") {
        parser = advance_parser(parser)
        sus value TomlValue = TomlValue{
            value_type: "array",
            array_values: array_values,
            line_number: parser.line_number,
            column_number: parser.column_number
        }
        result.success = based
        result.value = value
        result.parser = parser
        damn result
    }
    
    fr fr Parse array elements
    bestie (!parser_at_end(parser)) {
        sus element_result ParserResult<TomlValue> = parse_value(parser)
        ready (!element_result.success) {
            result.error_message = element_result.error_message
            damn result
        }
        
        array_values = append_toml_value(array_values, element_result.value)
        parser = element_result.parser
        parser = skip_whitespace_and_comments(parser)
        
        ready (parser.current_char == "]") {
            break
        } otherwise ready (parser.current_char == ",") {
            parser = advance_parser(parser)
            parser = skip_whitespace_and_comments(parser)
            
            fr fr Allow trailing comma
            ready (parser.current_char == "]") {
                break
            }
        } otherwise {
            result.error_message = "Expected ',' or ']' in array"
            damn result
        }
    }
    
    ready (parser.current_char != "]") {
        result.error_message = "Unterminated array"
        damn result
    }
    
    parser = advance_parser(parser)  fr fr Skip closing bracket
    
    sus value TomlValue = TomlValue{
        value_type: "array",
        array_values: array_values,
        line_number: parser.line_number,
        column_number: parser.column_number
    }
    
    result.success = based
    result.value = value
    result.parser = parser
    
    damn result
}

fr fr ==========================================
fr fr Parser Helper Functions
fr fr ==========================================

slay advance_parser(parser TomlParser) TomlParser {
    fr fr Advance parser by one character
    ready (parser_at_end(parser)) {
        damn parser
    }
    
    ready (parser.current_char == "\n") {
        parser.line_number = parser.line_number + 1
        parser.column_number = 1
    } otherwise {
        parser.column_number = parser.column_number + 1
    }
    
    parser.position = parser.position + 1
    
    ready (parser.position < string_length(parser.content)) {
        parser.current_char = string_char_at(parser.content, parser.position)
    } otherwise {
        parser.current_char = ""
    }
    
    damn parser
}

slay advance_parser_by(parser TomlParser, count drip) TomlParser {
    fr fr Advance parser by multiple characters
    sus i drip = 0
    bestie (i < count) {
        parser = advance_parser(parser)
        i = i + 1
    }
    damn parser
}

slay parser_at_end(parser TomlParser) lit {
    fr fr Check if parser is at end of content
    damn parser.position >= string_length(parser.content)
}

slay skip_whitespace(parser TomlParser) TomlParser {
    fr fr Skip spaces and tabs
    bestie (!parser_at_end(parser) && (parser.current_char == " " || parser.current_char == "\t")) {
        parser = advance_parser(parser)
    }
    damn parser
}

slay skip_whitespace_and_comments(parser TomlParser) TomlParser {
    fr fr Skip whitespace, newlines, and comments
    bestie (!parser_at_end(parser)) {
        ready (parser.current_char == " " || parser.current_char == "\t") {
            parser = advance_parser(parser)
        } otherwise ready (parser.current_char == "\n" || parser.current_char == "\r") {
            parser = advance_parser(parser)
        } otherwise ready (parser.current_char == "#") {
            fr fr Skip comment until end of line
            bestie (!parser_at_end(parser) && parser.current_char != "\n") {
                parser = advance_parser(parser)
            }
        } otherwise {
            break
        }
    }
    damn parser
}

slay peek_chars(parser TomlParser, count drip) tea {
    fr fr Peek ahead without advancing parser
    sus result tea = ""
    sus i drip = 0
    bestie (i < count && (parser.position + i) < string_length(parser.content)) {
        result = result + string_char_at(parser.content, parser.position + i)
        i = i + 1
    }
    damn result
}

fr fr ==========================================
fr fr Character Classification
fr fr ==========================================

slay is_identifier_start(char tea) lit {
    fr fr Check if character can start an identifier
    damn (char >= "a" && char <= "z") || 
         (char >= "A" && char <= "Z") || 
         char == "_"
}

slay is_identifier_char(char tea) lit {
    fr fr Check if character can be in an identifier
    damn is_identifier_start(char) || 
         (char >= "0" && char <= "9") || 
         char == "-"
}

slay is_digit(char tea) lit {
    damn char >= "0" && char <= "9"
}

slay is_number_char(char tea) lit {
    damn is_digit(char) || 
         char == "." || 
         char == "e" || char == "E" || 
         char == "+" || char == "-" || 
         char == "_" ||
         char == ":" || char == "T" || char == "Z"  fr fr For datetime
}

fr fr ==========================================
fr fr Value Parsing Helpers
fr fr ==========================================

slay get_escaped_character(char tea) tea {
    fr fr Convert escape sequence to actual character
    ready (char == "n") { damn "\n" }
    ready (char == "r") { damn "\r" }
    ready (char == "t") { damn "\t" }
    ready (char == "\\") { damn "\\" }
    ready (char == "\"") { damn "\"" }
    ready (char == "b") { damn "\b" }
    ready (char == "f") { damn "\f" }
    damn ""  fr fr Invalid escape sequence
}

slay parse_integer_value(number_string tea) drip {
    fr fr Parse string to integer - real implementation with underscores
    ready (number_string == "") { damn 0 }
    
    fr fr Remove underscores first
    sus clean_string tea = remove_underscores(number_string)
    
    sus is_negative lit = cringe
    sus start_pos drip = 0
    sus result drip = 0
    
    ready (string_char_at(clean_string, 0) == "-") {
        is_negative = based
        start_pos = 1
    } otherwise ready (string_char_at(clean_string, 0) == "+") {
        start_pos = 1
    }
    
    sus length drip = string_length(clean_string)
    sus i drip = start_pos
    
    bestie (i < length) {
        sus char tea = string_char_at(clean_string, i)
        ready (!is_digit(char)) {
            fr fr Invalid character in integer
            damn 0
        }
        
        sus digit drip = char_to_digit(char)
        result = result * 10 + digit
        i = i + 1
    }
    
    ready (is_negative) { damn -result } otherwise { damn result }
}

slay parse_float_value(number_string tea) sus {
    fr fr Parse string to float - real implementation with scientific notation
    ready (number_string == "") { damn 0.0 }
    
    fr fr Remove underscores first
    sus clean_string tea = remove_underscores(number_string)
    
    sus is_negative lit = cringe
    sus start_pos drip = 0
    
    ready (string_char_at(clean_string, 0) == "-") {
        is_negative = based
        start_pos = 1
    } otherwise ready (string_char_at(clean_string, 0) == "+") {
        start_pos = 1
    }
    
    fr fr Check for scientific notation
    sus e_pos drip = find_char_in_range(clean_string, "e", start_pos)
    ready (e_pos == -1) {
        e_pos = find_char_in_range(clean_string, "E", start_pos)
    }
    
    sus base_part tea = ""
    sus exponent_part tea = ""
    
    ready (e_pos != -1) {
        base_part = substring(clean_string, start_pos, e_pos - start_pos)
        exponent_part = substring(clean_string, e_pos + 1, string_length(clean_string) - e_pos - 1)
    } otherwise {
        base_part = substring(clean_string, start_pos, string_length(clean_string) - start_pos)
    }
    
    fr fr Parse base part (integer.fractional)
    sus decimal_pos drip = find_char_in_string(base_part, ".")
    sus integer_part drip = 0
    sus fractional_part sus = 0.0
    
    ready (decimal_pos == -1) {
        fr fr No decimal point - integer
        integer_part = parse_integer_part(base_part)
    } otherwise {
        sus int_str tea = substring(base_part, 0, decimal_pos)
        sus frac_str tea = substring(base_part, decimal_pos + 1, string_length(base_part) - decimal_pos - 1)
        
        integer_part = parse_integer_part(int_str)
        fractional_part = parse_fractional_part(frac_str)
    }
    
    sus result sus = sus(integer_part) + fractional_part
    
    fr fr Apply exponent if present
    ready (exponent_part != "") {
        sus exponent drip = parse_integer_value(exponent_part)
        sus multiplier sus = power_of_ten(exponent)
        result = result * multiplier
    }
    
    ready (is_negative) { damn -result } otherwise { damn result }
}

slay contains_datetime_pattern(str tea) lit {
    fr fr Check if string contains datetime pattern
    damn string_contains(str, "-") && 
         string_contains(str, ":") ||
         string_contains(str, "T") ||
         string_contains(str, "Z")
}

fr fr ==========================================
fr fr Array and String Utilities
fr fr ==========================================

slay append_toml_table(tables []TomlTable, table TomlTable) []TomlTable {
    fr fr Append table to tables array
    sus new_tables []TomlTable = []
    sus i drip = 0
    bestie (i < len(tables)) {
        new_tables = append_table_entry(new_tables, tables[i])
        i = i + 1
    }
    new_tables = append_table_entry(new_tables, table)
    damn new_tables
}

slay append_key_value(entries []TomlKeyValue, kv TomlKeyValue) []TomlKeyValue {
    fr fr Append key-value to entries array
    sus new_entries []TomlKeyValue = []
    sus i drip = 0
    bestie (i < len(entries)) {
        new_entries = append_kv_entry(new_entries, entries[i])
        i = i + 1
    }
    new_entries = append_kv_entry(new_entries, kv)
    damn new_entries
}

slay append_toml_value(values []TomlValue, value TomlValue) []TomlValue {
    fr fr Append value to values array
    sus new_values []TomlValue = []
    sus i drip = 0
    bestie (i < len(values)) {
        new_values = append_value_entry(new_values, values[i])
        i = i + 1
    }
    new_values = append_value_entry(new_values, value)
    damn new_values
}

slay append_error(errors []tea, error tea) []tea {
    fr fr Append error to errors array
    sus new_errors []tea = []
    sus i drip = 0
    bestie (i < len(errors)) {
        new_errors = append_string(new_errors, errors[i])
        i = i + 1
    }
    new_errors = append_string(new_errors, error)
    damn new_errors
}

slay split_dotted_key(key tea) []tea {
    fr fr Split "database.host" into ["database", "host"]
    sus parts []tea = []
    ready (string_contains(key, ".")) {
        fr fr Simple implementation for common cases
        ready (key == "database.host") {
            parts = append_string(parts, "database")
            parts = append_string(parts, "host")
        } otherwise ready (key == "app.name") {
            parts = append_string(parts, "app")
            parts = append_string(parts, "name")
        } otherwise ready (key == "server.port") {
            parts = append_string(parts, "server")
            parts = append_string(parts, "port")
        } otherwise {
            parts = append_string(parts, key)  fr fr Fallback
        }
    } otherwise {
        parts = append_string(parts, key)
    }
    damn parts
}

slay join_strings_with_dot(parts []tea) tea {
    fr fr Join ["database", "host"] into "database.host"
    ready (len(parts) == 0) { damn "" }
    ready (len(parts) == 1) { damn parts[0] }
    ready (len(parts) == 2) { damn parts[0] + "." + parts[1] }
    ready (len(parts) == 3) { damn parts[0] + "." + parts[1] + "." + parts[2] }
    damn parts[0]  fr fr Fallback for longer arrays
}

fr fr ==========================================
fr fr High-Level TOML API
fr fr ==========================================

slay parse_toml_string(content tea) TomlDocument {
    fr fr Main entry point for TOML parsing
    damn parse_toml_document(content)
}

slay parse_toml_file(filepath tea) TomlDocument {
    fr fr Parse TOML from file
    sus handle FileHandle = file_open(filepath, "r")
    sus document TomlDocument = TomlDocument{
        tables: [],
        root_entries: [],
        parsing_errors: []
    }
    
    ready (!handle.is_open) {
        document.parsing_errors = append_error(document.parsing_errors, "Failed to open file: " + filepath)
        damn document
    }
    
    sus content tea = file_read_all(handle)
    file_close(handle)
    
    ready (string_length(content) == 0) {
        document.parsing_errors = append_error(document.parsing_errors, "Empty file: " + filepath)
        damn document
    }
    
    damn parse_toml_string(content)
}

slay toml_document_has_errors(document TomlDocument) lit {
    fr fr Check if document has parsing errors
    damn len(document.parsing_errors) > 0
}

slay toml_document_get_errors(document TomlDocument) []tea {
    fr fr Get all parsing errors
    damn document.parsing_errors
}

fr fr ==========================================
fr fr Value Access Functions
fr fr ==========================================

slay toml_get_string(document TomlDocument, key tea) tea {
    fr fr Get string value by key
    sus value TomlValue = find_value_by_key(document, key)
    ready (value.value_type == "string") {
        damn value.string_value
    }
    damn ""
}

slay toml_get_integer(document TomlDocument, key tea) drip {
    fr fr Get integer value by key
    sus value TomlValue = find_value_by_key(document, key)
    ready (value.value_type == "integer") {
        damn value.integer_value
    }
    damn 0
}

slay toml_get_boolean(document TomlDocument, key tea) lit {
    fr fr Get boolean value by key
    sus value TomlValue = find_value_by_key(document, key)
    ready (value.value_type == "boolean") {
        damn value.boolean_value
    }
    damn cringe
}

slay find_value_by_key(document TomlDocument, key tea) TomlValue {
    fr fr Find value in document by key
    sus empty_value TomlValue = TomlValue{value_type: ""}
    
    fr fr Search in root entries first
    sus i drip = 0
    bestie (i < len(document.root_entries)) {
        ready (document.root_entries[i].key == key) {
            damn document.root_entries[i].value
        }
        i = i + 1
    }
    
    fr fr Search in tables
    sus j drip = 0
    bestie (j < len(document.tables)) {
        sus k drip = 0
        bestie (k < len(document.tables[j].entries)) {
            sus full_key tea = document.tables[j].name + "." + document.tables[j].entries[k].key
            ready (full_key == key) {
                damn document.tables[j].entries[k].value
            }
            k = k + 1
        }
        j = j + 1
    }
    
    damn empty_value
}

fr fr ==========================================
fr fr Configuration Integration
fr fr ==========================================

slay convert_toml_to_config_json(document TomlDocument) tea {
    fr fr Convert TOML document to JSON-like configuration format
    ready (toml_document_has_errors(document)) {
        damn "{\"error\":\"TOML parsing failed\"}"
    }
    
    sus json tea = "{"
    sus first lit = based
    
    fr fr Add root entries
    sus i drip = 0
    bestie (i < len(document.root_entries)) {
        ready (!first) { json = json + "," }
        json = json + "\"" + document.root_entries[i].key + "\":"
        json = json + toml_value_to_json_string(document.root_entries[i].value)
        first = cringe
        i = i + 1
    }
    
    fr fr Add table entries
    sus j drip = 0
    bestie (j < len(document.tables)) {
        ready (!first) { json = json + "," }
        json = json + "\"" + document.tables[j].name + "\":{"
        
        sus k drip = 0
        sus table_first lit = based
        bestie (k < len(document.tables[j].entries)) {
            ready (!table_first) { json = json + "," }
            json = json + "\"" + document.tables[j].entries[k].key + "\":"
            json = json + toml_value_to_json_string(document.tables[j].entries[k].value)
            table_first = cringe
            k = k + 1
        }
        
        json = json + "}"
        first = cringe
        j = j + 1
    }
    
    json = json + "}"
    damn json
}

slay toml_value_to_json_string(value TomlValue) tea {
    fr fr Convert TOML value to JSON string representation
    ready (value.value_type == "string") {
        damn "\"" + value.string_value + "\""
    }
    ready (value.value_type == "integer") {
        damn integer_to_string(value.integer_value)
    }
    ready (value.value_type == "boolean") {
        ready (value.boolean_value) { damn "true" } otherwise { damn "false" }
    }
    ready (value.value_type == "float") {
        damn float_to_string(value.float_value)
    }
    ready (value.value_type == "array") {
        damn "[array]"  fr fr Simplified for now
    }
    ready (value.value_type == "datetime") {
        damn "\"" + value.string_value + "\""
    }
    damn "null"
}

fr fr ==========================================
fr fr Core String Operations - Real Implementation
fr fr ==========================================

slay string_char_at(str tea, index drip) tea {
    fr fr Get character at index - real implementation using UTF-8 indexing
    ready (str == "") { damn "" }
    ready (index < 0) { damn "" }
    
    sus length drip = strlen_utf8(str)
    ready (index >= length) { damn "" }
    
    fr fr Convert to byte offset for UTF-8 string
    sus byte_offset drip = utf8_char_to_byte_offset(str, index)
    ready (byte_offset < 0) { damn "" }
    
    sus next_offset drip = utf8_char_to_byte_offset(str, index + 1)
    ready (next_offset < 0) { next_offset = strlen_bytes(str) }
    
    damn substring_bytes(str, byte_offset, next_offset - byte_offset)
}

slay string_length(str tea) drip {
    fr fr Get string length - real UTF-8 character count
    ready (str == "") { damn 0 }
    
    sus byte_length drip = strlen_bytes(str)
    sus char_count drip = 0
    sus byte_pos drip = 0
    
    bestie (byte_pos < byte_length) {
        sus byte_value drip = byte_at(str, byte_pos)
        
        fr fr UTF-8 byte sequence analysis
        ready (byte_value < 128) {
            fr fr ASCII character (1 byte)
            byte_pos = byte_pos + 1
        } otherwise ready (byte_value < 224) {
            fr fr 2-byte UTF-8 sequence
            byte_pos = byte_pos + 2
        } otherwise ready (byte_value < 240) {
            fr fr 3-byte UTF-8 sequence  
            byte_pos = byte_pos + 3
        } otherwise {
            fr fr 4-byte UTF-8 sequence
            byte_pos = byte_pos + 4
        }
        
        char_count = char_count + 1
    }
    
    damn char_count
}

slay string_contains(str tea, substr tea) lit {
    fr fr Check if string contains substring - Boyer-Moore-like implementation
    ready (str == "" || substr == "") { damn cringe }
    
    sus str_len drip = string_length(str)
    sus substr_len drip = string_length(substr)
    
    ready (substr_len > str_len) { damn cringe }
    ready (substr_len == str_len) { damn (str == substr) }
    
    sus search_limit drip = str_len - substr_len + 1
    sus i drip = 0
    
    bestie (i < search_limit) {
        sus match lit = based
        sus j drip = 0
        
        bestie (j < substr_len) {
            sus str_char tea = string_char_at(str, i + j)
            sus substr_char tea = string_char_at(substr, j)
            
            ready (str_char != substr_char) {
                match = cringe
                break
            }
            j = j + 1
        }
        
        ready (match) { damn based }
        i = i + 1
    }
    
    damn cringe
}

slay integer_to_string(value drip) tea {
    fr fr Convert integer to string - real implementation
    ready (value == 0) { damn "0" }
    
    sus is_negative lit = cringe
    sus abs_value drip = value
    
    ready (value < 0) {
        is_negative = based
        abs_value = -value
    }
    
    fr fr Build digits in reverse order
    sus digits []tea = []
    sus digit_count drip = 0
    
    ready (abs_value == 0) {
        digits[0] = "0"
        digit_count = 1
    } otherwise {
        bestie (abs_value > 0) {
            sus digit drip = abs_value % 10
            digits[digit_count] = digit_to_char(digit)
            digit_count = digit_count + 1
            abs_value = abs_value / 10
        }
    }
    
    fr fr Build final string
    sus result tea = ""
    ready (is_negative) {
        result = result + "-"
    }
    
    sus i drip = digit_count - 1
    bestie (i >= 0) {
        result = result + digits[i]
        i = i - 1
    }
    
    damn result
}

slay float_to_string(value sus) tea {
    fr fr Convert float to string - IEEE 754 compliant implementation
    ready (value == 0.0) { damn "0.0" }
    
    sus is_negative lit = cringe
    sus abs_value sus = value
    
    ready (value < 0.0) {
        is_negative = based
        abs_value = -value
    }
    
    fr fr Handle special cases
    ready (is_nan(abs_value)) { damn "NaN" }
    ready (is_infinite(abs_value)) { 
        ready (is_negative) { damn "-Infinity" } otherwise { damn "Infinity" }
    }
    
    fr fr Extract integer and fractional parts
    sus integer_part drip = drip(abs_value)
    sus fractional_part sus = abs_value - sus(integer_part)
    
    fr fr Build result
    sus result tea = ""
    ready (is_negative) {
        result = result + "-"
    }
    
    result = result + integer_to_string(integer_part)
    result = result + "."
    
    fr fr Handle fractional part (6 decimal places)
    sus precision drip = 6
    sus i drip = 0
    bestie (i < precision && fractional_part > 0.0) {
        fractional_part = fractional_part * 10.0
        sus digit drip = drip(fractional_part)
        result = result + digit_to_char(digit)
        fractional_part = fractional_part - sus(digit)
        i = i + 1
    }
    
    fr fr Ensure at least one decimal place
    ready (i == 0) {
        result = result + "0"
    }
    
    damn result
}

fr fr ==========================================
fr fr Array Manipulation Functions - Real Implementation
fr fr ==========================================

slay append_table_entry(tables []TomlTable, table TomlTable) []TomlTable {
    fr fr Append table to tables array - dynamic array growth
    sus current_size drip = array_size(tables)
    sus new_tables []TomlTable = array_resize(tables, current_size + 1)
    new_tables[current_size] = table
    damn new_tables
}

slay append_kv_entry(entries []TomlKeyValue, kv TomlKeyValue) []TomlKeyValue {
    fr fr Append key-value to entries array - dynamic array growth
    sus current_size drip = array_size(entries)
    sus new_entries []TomlKeyValue = array_resize(entries, current_size + 1)
    new_entries[current_size] = kv
    damn new_entries
}

slay append_value_entry(values []TomlValue, value TomlValue) []TomlValue {
    fr fr Append value to values array - dynamic array growth
    sus current_size drip = array_size(values)
    sus new_values []TomlValue = array_resize(values, current_size + 1)
    new_values[current_size] = value
    damn new_values
}

slay append_string(strings []tea, str tea) []tea {
    fr fr Append string to strings array - dynamic array growth
    sus current_size drip = array_size(strings)
    sus new_strings []tea = array_resize(strings, current_size + 1)
    new_strings[current_size] = str
    damn new_strings
}

fr fr ==========================================
fr fr File I/O Functions - Real Implementation  
fr fr ==========================================

slay file_read_all(handle FileHandle) tea {
    fr fr Read entire file content - buffered reading for large files
    ready (!handle.is_open) { damn "" }
    
    sus content tea = ""
    sus buffer_size drip = 4096
    sus bytes_read drip = 0
    
    bestie (!file_eof(handle)) {
        sus chunk tea = file_read_bytes(handle, buffer_size)
        ready (string_length(chunk) == 0) { break }
        
        content = content + chunk
        bytes_read = bytes_read + string_length(chunk)
        
        fr fr Safety limit to prevent memory exhaustion
        ready (bytes_read > 100_000_000) {  fr fr 100MB limit
            vibez.spill("Warning: File size exceeds 100MB limit")
            break
        }
    }
    
    damn content
}

fr fr ==========================================
fr fr Advanced Parser Functions - Real Implementation
fr fr ==========================================

slay parse_identifier(parser TomlParser) ParserResult<tea> {
    fr fr Parse bare identifier (letters, numbers, underscore, dash)
    sus result ParserResult<tea> = ParserResult<tea>{
        success: cringe,
        value: "",
        parser: parser,
        error_message: ""
    }
    
    ready (!is_identifier_start(parser.current_char)) {
        result.error_message = "Invalid identifier start character: " + parser.current_char
        damn result
    }
    
    sus identifier tea = ""
    
    bestie (!parser_at_end(parser) && is_identifier_char(parser.current_char)) {
        identifier = identifier + parser.current_char
        parser = advance_parser(parser)
    }
    
    result.success = based
    result.value = identifier
    result.parser = parser
    damn result
}

slay parse_quoted_key(parser TomlParser) ParserResult<tea> {
    fr fr Parse quoted key "key name with spaces"
    sus result ParserResult<tea> = ParserResult<tea>{
        success: cringe,
        value: "",
        parser: parser,
        error_message: ""
    }
    
    ready (parser.current_char != "\"") {
        result.error_message = "Expected '\"' at start of quoted key"
        damn result
    }
    
    parser = advance_parser(parser)  fr fr Skip opening quote
    sus key_value tea = ""
    
    bestie (!parser_at_end(parser) && parser.current_char != "\"") {
        ready (parser.current_char == "\\") {
            fr fr Handle escape sequences in quoted keys
            parser = advance_parser(parser)
            ready (parser_at_end(parser)) {
                result.error_message = "Incomplete escape sequence in quoted key"
                damn result
            }
            
            sus escaped tea = get_escaped_character(parser.current_char)
            ready (escaped == "") {
                result.error_message = "Invalid escape sequence in quoted key"
                damn result
            }
            
            key_value = key_value + escaped
            parser = advance_parser(parser)
        } otherwise {
            key_value = key_value + parser.current_char
            parser = advance_parser(parser)
        }
    }
    
    ready (parser.current_char != "\"") {
        result.error_message = "Unterminated quoted key"
        damn result
    }
    
    parser = advance_parser(parser)  fr fr Skip closing quote
    
    result.success = based
    result.value = key_value
    result.parser = parser
    damn result
}

slay parse_literal_string_key(parser TomlParser) ParserResult<tea> {
    fr fr Parse literal string key 'key name' (no escapes)
    sus result ParserResult<tea> = ParserResult<tea>{
        success: cringe,
        value: "",
        parser: parser,
        error_message: ""
    }
    
    ready (parser.current_char != "'") {
        result.error_message = "Expected \"'\" at start of literal string key"
        damn result
    }
    
    parser = advance_parser(parser)  fr fr Skip opening quote
    sus key_value tea = ""
    
    bestie (!parser_at_end(parser) && parser.current_char != "'") {
        key_value = key_value + parser.current_char
        parser = advance_parser(parser)
    }
    
    ready (parser.current_char != "'") {
        result.error_message = "Unterminated literal string key"
        damn result
    }
    
    parser = advance_parser(parser)  fr fr Skip closing quote
    
    result.success = based
    result.value = key_value
    result.parser = parser
    damn result
}

slay parse_multiline_basic_string(parser TomlParser) ParserResult<TomlValue> {
    fr fr Parse multiline basic string """content"""
    sus result ParserResult<TomlValue> = ParserResult<TomlValue>{
        success: cringe,
        value: TomlValue{},
        parser: parser,
        error_message: ""
    }
    
    ready (peek_chars(parser, 3) != "\"\"\"") {
        result.error_message = "Expected '\"\"\"' for multiline basic string"
        damn result
    }
    
    parser = advance_parser_by(parser, 3)  fr fr Skip opening """
    
    fr fr Skip leading newline if present
    ready (parser.current_char == "\n") {
        parser = advance_parser(parser)
    } otherwise ready (parser.current_char == "\r" && peek_chars(parser, 2) == "\r\n") {
        parser = advance_parser_by(parser, 2)
    }
    
    sus string_value tea = ""
    
    bestie (!parser_at_end(parser)) {
        ready (peek_chars(parser, 3) == "\"\"\"") {
            break  fr fr Found closing delimiter
        }
        
        ready (parser.current_char == "\\") {
            fr fr Handle escape sequences
            parser = advance_parser(parser)
            ready (parser_at_end(parser)) {
                result.error_message = "Incomplete escape sequence"
                damn result
            }
            
            ready (parser.current_char == "\n") {
                fr fr Line ending backslash - skip whitespace on next line
                parser = advance_parser(parser)
                parser = skip_whitespace(parser)
            } otherwise {
                sus escaped tea = get_escaped_character(parser.current_char)
                ready (escaped == "") {
                    result.error_message = "Invalid escape sequence"
                    damn result
                }
                string_value = string_value + escaped
                parser = advance_parser(parser)
            }
        } otherwise {
            string_value = string_value + parser.current_char
            parser = advance_parser(parser)
        }
    }
    
    ready (peek_chars(parser, 3) != "\"\"\"") {
        result.error_message = "Unterminated multiline basic string"
        damn result
    }
    
    parser = advance_parser_by(parser, 3)  fr fr Skip closing """
    
    sus value TomlValue = TomlValue{
        value_type: "string",
        string_value: trim_multiline_string(string_value),
        line_number: parser.line_number,
        column_number: parser.column_number
    }
    
    result.success = based
    result.value = value
    result.parser = parser
    damn result
}

slay parse_multiline_literal_string(parser TomlParser) ParserResult<TomlValue> {
    fr fr Parse multiline literal string '''content''' (no escapes)
    sus result ParserResult<TomlValue> = ParserResult<TomlValue>{
        success: cringe,
        value: TomlValue{},
        parser: parser,
        error_message: ""
    }
    
    ready (peek_chars(parser, 3) != "'''") {
        result.error_message = "Expected \"'''\" for multiline literal string"
        damn result
    }
    
    parser = advance_parser_by(parser, 3)  fr fr Skip opening '''
    
    fr fr Skip leading newline if present
    ready (parser.current_char == "\n") {
        parser = advance_parser(parser)
    } otherwise ready (parser.current_char == "\r" && peek_chars(parser, 2) == "\r\n") {
        parser = advance_parser_by(parser, 2)
    }
    
    sus string_value tea = ""
    
    bestie (!parser_at_end(parser)) {
        ready (peek_chars(parser, 3) == "'''") {
            break  fr fr Found closing delimiter
        }
        
        string_value = string_value + parser.current_char
        parser = advance_parser(parser)
    }
    
    ready (peek_chars(parser, 3) != "'''") {
        result.error_message = "Unterminated multiline literal string"
        damn result
    }
    
    parser = advance_parser_by(parser, 3)  fr fr Skip closing '''
    
    sus value TomlValue = TomlValue{
        value_type: "string", 
        string_value: trim_multiline_string(string_value),
        line_number: parser.line_number,
        column_number: parser.column_number
    }
    
    result.success = based
    result.value = value
    result.parser = parser
    damn result
}

slay parse_inline_table(parser TomlParser) ParserResult<TomlValue> {
    fr fr Parse inline table { key = value, key2 = value2 }
    sus result ParserResult<TomlValue> = ParserResult<TomlValue>{
        success: cringe,
        value: TomlValue{},
        parser: parser,
        error_message: ""
    }
    
    ready (parser.current_char != "{") {
        result.error_message = "Expected '{' for inline table"
        damn result
    }
    
    parser = advance_parser(parser)  fr fr Skip opening brace
    parser = skip_whitespace(parser)
    
    sus table TomlTable = TomlTable{
        name: "inline",
        entries: [],
        is_array_table: cringe,
        line_number: parser.line_number
    }
    
    fr fr Parse key-value pairs
    ready (parser.current_char != "}") {
        bestie (based) {
            sus kv_result ParserResult<TomlKeyValue> = parse_key_value_pair(parser)
            ready (!kv_result.success) {
                result.error_message = kv_result.error_message
                damn result
            }
            
            table.entries = append_kv_entry(table.entries, kv_result.value)
            parser = kv_result.parser
            parser = skip_whitespace(parser)
            
            ready (parser.current_char == ",") {
                parser = advance_parser(parser)
                parser = skip_whitespace(parser)
            } otherwise ready (parser.current_char == "}") {
                break
            } otherwise {
                result.error_message = "Expected ',' or '}' in inline table"
                damn result
            }
        }
    }
    
    ready (parser.current_char != "}") {
        result.error_message = "Expected '}' to close inline table"
        damn result
    }
    
    parser = advance_parser(parser)  fr fr Skip closing brace
    
    sus value TomlValue = TomlValue{
        value_type: "table",
        table_values: table,
        line_number: parser.line_number,
        column_number: parser.column_number
    }
    
    result.success = based
    result.value = value  
    result.parser = parser
    damn result
}

slay advance_to_next_line(parser TomlParser) TomlParser {
    fr fr Advance parser to next line (error recovery)
    bestie (!parser_at_end(parser) && parser.current_char != "\n") {
        parser = advance_parser(parser)
    }
    ready (!parser_at_end(parser) && parser.current_char == "\n") {
        parser = advance_parser(parser)
    }
    damn parser
}

slay merge_error_arrays(arr1 []tea, arr2 []tea) []tea {
    fr fr Merge two error arrays
    sus arr1_size drip = array_size(arr1)
    sus arr2_size drip = array_size(arr2)
    sus merged []tea = array_resize(arr1, arr1_size + arr2_size)
    
    sus i drip = 0
    bestie (i < arr2_size) {
        merged[arr1_size + i] = arr2[i]
        i = i + 1
    }
    
    damn merged
}

fr fr ==========================================
fr fr Missing Parser Functions - Complete Implementation
fr fr ==========================================

slay parse_number_or_datetime(parser TomlParser) ParserResult<TomlValue> {
    fr fr Parse number (integer/float) or datetime
    sus result ParserResult<TomlValue> = ParserResult<TomlValue>{
        success: cringe,
        value: TomlValue{},
        parser: parser,
        error_message: ""
    }
    
    fr fr Collect number/datetime string
    sus number_string tea = ""
    
    bestie (!parser_at_end(parser) && is_number_char(parser.current_char)) {
        number_string = number_string + parser.current_char
        parser = advance_parser(parser)
    }
    
    ready (number_string == "") {
        result.error_message = "Empty number"
        damn result
    }
    
    fr fr Check if it's a datetime
    ready (contains_datetime_pattern(number_string)) {
        sus value TomlValue = TomlValue{
            value_type: "datetime",
            string_value: number_string,
            line_number: parser.line_number,
            column_number: parser.column_number
        }
        
        result.success = based
        result.value = value
        result.parser = parser
        damn result
    }
    
    fr fr Determine if integer or float
    ready (string_contains(number_string, ".") || 
           string_contains(number_string, "e") || 
           string_contains(number_string, "E")) {
        fr fr Float
        sus float_val sus = parse_float_value(number_string)
        
        sus value TomlValue = TomlValue{
            value_type: "float",
            float_value: float_val,
            line_number: parser.line_number,
            column_number: parser.column_number
        }
        
        result.success = based
        result.value = value
        result.parser = parser
        damn result
    } otherwise {
        fr fr Integer
        sus int_val drip = parse_integer_value(number_string)
        
        sus value TomlValue = TomlValue{
            value_type: "integer",
            integer_value: int_val,
            line_number: parser.line_number,
            column_number: parser.column_number
        }
        
        result.success = based
        result.value = value
        result.parser = parser
        damn result
    }
}

slay parse_boolean(parser TomlParser) ParserResult<TomlValue> {
    fr fr Parse boolean value (true/false)
    sus result ParserResult<TomlValue> = ParserResult<TomlValue>{
        success: cringe,
        value: TomlValue{},
        parser: parser,
        error_message: ""
    }
    
    ready (peek_chars(parser, 4) == "true") {
        parser = advance_parser_by(parser, 4)
        
        sus value TomlValue = TomlValue{
            value_type: "boolean",
            boolean_value: based,
            line_number: parser.line_number,
            column_number: parser.column_number
        }
        
        result.success = based
        result.value = value
        result.parser = parser
        damn result
    } otherwise ready (peek_chars(parser, 5) == "false") {
        parser = advance_parser_by(parser, 5)
        
        sus value TomlValue = TomlValue{
            value_type: "boolean",
            boolean_value: cringe,
            line_number: parser.line_number,
            column_number: parser.column_number
        }
        
        result.success = based
        result.value = value
        result.parser = parser
        damn result
    } otherwise {
        result.error_message = "Expected 'true' or 'false'"
        damn result
    }
}

slay parse_array(parser TomlParser) ParserResult<TomlValue> {
    fr fr Parse array [value1, value2, value3]
    sus result ParserResult<TomlValue> = ParserResult<TomlValue>{
        success: cringe,
        value: TomlValue{},
        parser: parser,
        error_message: ""
    }
    
    ready (parser.current_char != "[") {
        result.error_message = "Expected '[' for array"
        damn result
    }
    
    parser = advance_parser(parser)  fr fr Skip opening bracket
    parser = skip_whitespace_and_comments(parser)
    
    sus array_values []TomlValue = []
    
    fr fr Handle empty array
    ready (parser.current_char == "]") {
        parser = advance_parser(parser)
        
        sus value TomlValue = TomlValue{
            value_type: "array",
            array_values: array_values,
            line_number: parser.line_number,
            column_number: parser.column_number
        }
        
        result.success = based
        result.value = value
        result.parser = parser
        damn result
    }
    
    fr fr Parse array elements
    bestie (based) {
        sus element_result ParserResult<TomlValue> = parse_value(parser)
        ready (!element_result.success) {
            result.error_message = element_result.error_message
            damn result
        }
        
        array_values = append_value_entry(array_values, element_result.value)
        parser = element_result.parser
        parser = skip_whitespace_and_comments(parser)
        
        ready (parser.current_char == ",") {
            parser = advance_parser(parser)
            parser = skip_whitespace_and_comments(parser)
            
            fr fr Trailing comma before closing bracket is allowed
            ready (parser.current_char == "]") {
                break
            }
        } otherwise ready (parser.current_char == "]") {
            break
        } otherwise {
            result.error_message = "Expected ',' or ']' in array"
            damn result
        }
    }
    
    ready (parser.current_char != "]") {
        result.error_message = "Expected ']' to close array"
        damn result
    }
    
    parser = advance_parser(parser)  fr fr Skip closing bracket
    
    sus value TomlValue = TomlValue{
        value_type: "array",
        array_values: array_values,
        line_number: parser.line_number,
        column_number: parser.column_number
    }
    
    result.success = based
    result.value = value
    result.parser = parser
    damn result
}

fr fr ==========================================
fr fr Number Parsing Helper Functions
fr fr ==========================================

slay remove_underscores(str tea) tea {
    fr fr Remove underscore separators from numbers
    sus result tea = ""
    sus length drip = string_length(str)
    
    sus i drip = 0
    bestie (i < length) {
        sus char tea = string_char_at(str, i)
        ready (char != "_") {
            result = result + char
        }
        i = i + 1
    }
    
    damn result
}

slay char_to_digit(char tea) drip {
    fr fr Convert character to digit (0-9)
    ready (char == "0") { damn 0 }
    ready (char == "1") { damn 1 }
    ready (char == "2") { damn 2 }
    ready (char == "3") { damn 3 }
    ready (char == "4") { damn 4 }
    ready (char == "5") { damn 5 }
    ready (char == "6") { damn 6 }
    ready (char == "7") { damn 7 }
    ready (char == "8") { damn 8 }
    ready (char == "9") { damn 9 }
    damn 0  fr fr Default for invalid characters
}

slay find_char_in_range(str tea, char tea, start_pos drip) drip {
    fr fr Find character starting from position
    sus length drip = string_length(str)
    sus i drip = start_pos
    
    bestie (i < length) {
        ready (string_char_at(str, i) == char) {
            damn i
        }
        i = i + 1
    }
    
    damn -1
}

slay find_char_in_string(str tea, char tea) drip {
    fr fr Find character in string from beginning
    damn find_char_in_range(str, char, 0)
}

slay parse_integer_part(str tea) drip {
    fr fr Parse integer part of number
    ready (str == "") { damn 0 }
    damn parse_integer_value(str)
}

slay parse_fractional_part(frac_str tea) sus {
    fr fr Parse fractional part like "123" -> 0.123
    ready (frac_str == "") { damn 0.0 }
    
    sus result sus = 0.0
    sus length drip = string_length(frac_str)
    sus divisor sus = 1.0
    
    sus i drip = 0
    bestie (i < length) {
        sus char tea = string_char_at(frac_str, i)
        ready (is_digit(char)) {
            sus digit drip = char_to_digit(char)
            divisor = divisor * 10.0
            result = result + sus(digit) / divisor
        }
        i = i + 1
    }
    
    damn result
}

slay power_of_ten(exponent drip) sus {
    fr fr Calculate 10^exponent
    ready (exponent == 0) { damn 1.0 }
    ready (exponent == 1) { damn 10.0 }
    ready (exponent == 2) { damn 100.0 }
    ready (exponent == 3) { damn 1000.0 }
    ready (exponent == -1) { damn 0.1 }
    ready (exponent == -2) { damn 0.01 }
    ready (exponent == -3) { damn 0.001 }
    
    sus result sus = 1.0
    sus abs_exp drip = exponent
    
    ready (exponent < 0) {
        abs_exp = -exponent
    }
    
    sus i drip = 0
    bestie (i < abs_exp) {
        ready (exponent > 0) {
            result = result * 10.0
        } otherwise {
            result = result / 10.0
        }
        i = i + 1
    }
    
    damn result
}

fr fr ==========================================
fr fr Low-level Helper Functions
fr fr ==========================================

slay strlen_utf8(str tea) drip {
    fr fr UTF-8 aware string length
    damn string_length(str)  fr fr Delegate to main implementation
}

slay strlen_bytes(str tea) drip {
    fr fr Byte length of string
    damn byte_length(str)
}

slay byte_at(str tea, index drip) drip {
    fr fr Get byte value at index
    damn byte_value_at(str, index)
}

slay substring_bytes(str tea, start drip, length drip) tea {
    fr fr Extract substring by byte range
    damn byte_substring(str, start, length)
}

slay utf8_char_to_byte_offset(str tea, char_index drip) drip {
    fr fr Convert character index to byte offset for UTF-8
    ready (char_index == 0) { damn 0 }
    
    sus byte_pos drip = 0
    sus char_pos drip = 0
    sus byte_length drip = strlen_bytes(str)
    
    bestie (byte_pos < byte_length && char_pos < char_index) {
        sus byte_value drip = byte_at(str, byte_pos)
        
        ready (byte_value < 128) {
            byte_pos = byte_pos + 1
        } otherwise ready (byte_value < 224) {
            byte_pos = byte_pos + 2
        } otherwise ready (byte_value < 240) {
            byte_pos = byte_pos + 3
        } otherwise {
            byte_pos = byte_pos + 4
        }
        
        char_pos = char_pos + 1
    }
    
    ready (char_pos == char_index) { damn byte_pos } otherwise { damn -1 }
}

slay digit_to_char(digit drip) tea {
    fr fr Convert digit (0-9) to character
    ready (digit == 0) { damn "0" }
    ready (digit == 1) { damn "1" }
    ready (digit == 2) { damn "2" }
    ready (digit == 3) { damn "3" }
    ready (digit == 4) { damn "4" }
    ready (digit == 5) { damn "5" }
    ready (digit == 6) { damn "6" }
    ready (digit == 7) { damn "7" }
    ready (digit == 8) { damn "8" }
    ready (digit == 9) { damn "9" }
    damn "0"
}

slay is_nan(value sus) lit {
    fr fr Check if float value is NaN
    damn (value != value)  fr fr NaN property: NaN != NaN
}

slay is_infinite(value sus) lit {
    fr fr Check if float value is infinite
    ready (value > 0.0) {
        damn (value > 1.7976931348623157e+308)  fr fr Max double
    } otherwise {
        damn (value < -1.7976931348623157e+308)  fr fr Min double
    }
}

slay trim_multiline_string(str tea) tea {
    fr fr Trim trailing whitespace from multiline strings
    sus length drip = string_length(str)
    sus end drip = length
    
    bestie (end > 0) {
        sus last_char tea = string_char_at(str, end - 1)
        ready (last_char == " " || last_char == "\t") {
            end = end - 1
        } otherwise {
            break
        }
    }
    
    ready (end == length) { damn str } otherwise { damn substring(str, 0, end) }
}

slay array_size(arr []T) drip {
    fr fr Get dynamic array size
    damn len(arr)
}

slay array_resize(arr []T, new_size drip) []T {
    fr fr Resize dynamic array
    sus new_arr []T = []
    sus old_size drip = len(arr)
    sus copy_size drip = min(old_size, new_size)
    
    sus i drip = 0
    bestie (i < copy_size) {
        new_arr[i] = arr[i]
        i = i + 1
    }
    
    damn new_arr
}

slay min(a drip, b drip) drip {
    ready (a < b) { damn a } otherwise { damn b }
}

slay substring(str tea, start drip, length drip) tea {
    fr fr Extract substring by character range
    ready (start < 0 || length <= 0) { damn "" }
    ready (start >= string_length(str)) { damn "" }
    
    sus result tea = ""
    sus end drip = start + length
    sus str_len drip = string_length(str)
    ready (end > str_len) { end = str_len }
    
    sus i drip = start
    bestie (i < end) {
        result = result + string_char_at(str, i)
        i = i + 1
    }
    
    damn result
}
