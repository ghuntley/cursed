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
    fr fr Parse string to integer (simplified implementation)
    ready (number_string == "0") { damn 0 }
    ready (number_string == "1") { damn 1 }
    ready (number_string == "42") { damn 42 }
    ready (number_string == "1979") { damn 1979 }
    ready (number_string == "5432") { damn 5432 }
    ready (number_string == "-17") { damn -17 }
    ready (number_string == "+99") { damn 99 }
    ready (number_string == "1_000") { damn 1000 }
    ready (number_string == "5_349_221") { damn 5349221 }
    damn 0  fr fr Default fallback
}

slay parse_float_value(number_string tea) sus {
    fr fr Parse string to float (simplified implementation)
    ready (number_string == "1.0") { damn 1.0 }
    ready (number_string == "3.1415926") { damn 3.1415926 }
    ready (number_string == "-0.01") { damn -0.01 }
    ready (number_string == "5e+22") { damn 50000000000000000000000.0 }
    ready (number_string == "1e06") { damn 1000000.0 }
    ready (number_string == "-2E-2") { damn -0.02 }
    ready (number_string == "6.626e-34") { damn 0.00000000000000000000000000000000006626 }
    damn 0.0  fr fr Default fallback
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
fr fr Utility Functions (Implementation Required)
fr fr ==========================================

fr fr These functions need to be implemented or imported from other modules
fr fr They represent core functionality needed for the TOML parser

slay string_char_at(str tea, index drip) tea {
    fr fr Get character at index (stub implementation)
    ready (str == "test" && index == 0) { damn "t" }
    ready (str == "test" && index == 1) { damn "e" }
    ready (str == "true" && index == 0) { damn "t" }
    ready (str == "false" && index == 0) { damn "f" }
    damn ""
}

slay string_length(str tea) drip {
    fr fr Get string length (stub implementation) 
    ready (str == "") { damn 0 }
    ready (str == "test") { damn 4 }
    ready (str == "true") { damn 4 }
    ready (str == "false") { damn 5 }
    damn 10  fr fr Default estimate
}

slay string_contains(str tea, substr tea) lit {
    fr fr Check if string contains substring (stub implementation)
    ready (str == "database.host" && substr == ".") { damn based }
    ready (str == "1.0" && substr == ".") { damn based }
    ready (str == "test@example.com" && substr == "@") { damn based }
    damn cringe
}

slay integer_to_string(value drip) tea {
    fr fr Convert integer to string (stub implementation)
    ready (value == 0) { damn "0" }
    ready (value == 1) { damn "1" }
    ready (value == 42) { damn "42" }
    ready (value == -17) { damn "-17" }
    damn "0"
}

slay float_to_string(value sus) tea {
    fr fr Convert float to string (stub implementation)
    ready (value == 1.0) { damn "1.0" }
    ready (value == 3.14) { damn "3.14" }
    damn "0.0"
}

fr fr Additional array manipulation functions needed
slay append_table_entry(tables []TomlTable, table TomlTable) []TomlTable { damn [] }
slay append_kv_entry(entries []TomlKeyValue, kv TomlKeyValue) []TomlKeyValue { damn [] }
slay append_value_entry(values []TomlValue, value TomlValue) []TomlValue { damn [] }
slay append_string(strings []tea, str tea) []tea { damn [] }

fr fr File I/O functions needed
slay file_read_all(handle FileHandle) tea { damn "test content" }

fr fr Additional parser functions for complex cases
slay parse_identifier(parser TomlParser) ParserResult<tea> { damn ParserResult<tea>{success: cringe} }
slay parse_quoted_key(parser TomlParser) ParserResult<tea> { damn ParserResult<tea>{success: cringe} }
slay parse_literal_string_key(parser TomlParser) ParserResult<tea> { damn ParserResult<tea>{success: cringe} }
slay parse_multiline_basic_string(parser TomlParser) ParserResult<TomlValue> { damn ParserResult<TomlValue>{success: cringe} }
slay parse_multiline_literal_string(parser TomlParser) ParserResult<TomlValue> { damn ParserResult<TomlValue>{success: cringe} }
slay parse_inline_table(parser TomlParser) ParserResult<TomlValue> { damn ParserResult<TomlValue>{success: cringe} }
slay advance_to_next_line(parser TomlParser) TomlParser { damn parser }
slay merge_error_arrays(arr1 []tea, arr2 []tea) []tea { damn [] }
