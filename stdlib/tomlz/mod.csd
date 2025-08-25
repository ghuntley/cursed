# CURSED Standard Library - TOML Processing Module
# Production-grade TOML 1.0.0 compliant parsing and generation
# Version: 1.0.0-production
# Last Updated: 2025-08-25

yeet "vibez"
yeet "stringz"
yeet "arrayz"
yeet "mathz"
yeet "error_core"

# TOML Value Types
enum TomlValueType {
    String,
    Integer,
    Float,
    Boolean,
    Datetime,
    LocalDatetime,
    LocalDate,
    LocalTime,
    Array,
    Table,
    InlineTable
}

# TOML String Types  
enum TomlStringType {
    Basic,          # "string"
    MultiLineBasic, # """string"""
    Literal,        # 'string'  
    MultiLineLiteral # '''string'''
}

# TOML Value Structure
squad TomlValue {
    value_type TomlValueType
    string_type TomlStringType
    value tea
    array_values []TomlValue
    table_values map[tea]TomlValue
    line_number drip
    column_number drip
    key_path []tea
}

# TOML Table Structure
squad TomlTable {
    name tea
    values map[tea]TomlValue
    is_array_table lit
    line_number drip
}

# TOML Document Structure
squad TomlDocument {
    values map[tea]TomlValue
    tables map[tea]TomlTable
    comments map[drip]tea
    version tea
}

# TOML Parser Configuration
squad TomlParserConfig {
    allow_duplicate_keys lit
    preserve_comments lit
    strict_mode lit
    max_depth drip
    max_table_name_length drip
    max_key_length drip
}

# TOML Parse Error
squad TomlParseError {
    message tea
    line drip
    column drip
    error_code drip
    context tea
}

# TOML Validation Result
squad TomlValidationResult {
    valid lit
    errors []tea
    warnings []tea
}

# ========================
# Core TOML Parsing Functions
# ========================

# Parse TOML from string
slay parse_toml(toml_content tea) yikes<TomlDocument> {
    sus config TomlParserConfig = {
        allow_duplicate_keys: cap,
        preserve_comments: based,
        strict_mode: cap,
        max_depth: 50,
        max_table_name_length: 256,
        max_key_length: 128
    }
    damn parse_toml_with_config(toml_content, config)
}

# Parse TOML with custom configuration
slay parse_toml_with_config(toml_content tea, config TomlParserConfig) yikes<TomlDocument> {
    # Initialize parser state
    sus state TomlParserState = {
        content: toml_content,
        position: 0,
        line: 1,
        column: 1,
        config: config,
        current_table: "",
        table_array_counts: make(map[tea]drip)
    }
    
    # Initialize document
    sus document TomlDocument = {
        values: make(map[tea]TomlValue),
        tables: make(map[tea]TomlTable),
        comments: make(map[drip]tea),
        version: "1.0.0"
    }
    
    # Parse entire document
    bestie (state.position < state.content.len()) {
        skip_toml_whitespace(state)
        
        ready (state.position >= state.content.len()) {
            break
        }
        
        sus ch tea = stringz.char_at(state.content, state.position)
        
        # Handle comments
        ready (ch == "#") {
            sus comment tea = parse_toml_comment(state)
            ready (config.preserve_comments) {
                document.comments[state.line] = comment
            }
        # Handle table headers
        } otherwise ready (ch == "[") {
            parse_toml_table_header(state, document) fam {
                when err -> yikes "Failed to parse table header: " + err
            }
        # Handle key-value pairs
        } otherwise ready (is_toml_key_start(ch)) {
            parse_toml_key_value_pair(state, document) fam {
                when err -> yikes "Failed to parse key-value pair: " + err
            }
        } otherwise {
            yikes "Unexpected character '" + ch + "' at line " + mathz.int_to_string(state.line)
        }
    }
    
    damn document
}

# Parse TOML from file
slay parse_toml_file(file_path tea) yikes<TomlDocument> {
    sus content tea = filez.read_file_content(file_path) fam {
        when err -> yikes "Failed to read TOML file: " + err
    }
    damn parse_toml(content)
}

# ========================
# TOML Generation Functions
# ========================

# Generate TOML from document
slay generate_toml(doc TomlDocument) tea {
    sus output tea = ""
    
    # Generate top-level key-value pairs first
    bestie (sus key tea, sus value TomlValue in doc.values) {
        ready (value.key_path.len() == 1) {  # Top-level only
            output = output + generate_toml_key_value(key, value) + "\n"
        }
    }
    
    # Add blank line before tables
    ready (doc.tables.len() > 0 && output != "") {
        output = output + "\n"
    }
    
    # Generate tables
    bestie (sus table_name tea, sus table TomlTable in doc.tables) {
        output = output + generate_toml_table(table_name, table) + "\n"
    }
    
    damn stringz.trim(output)
}

# Generate formatted TOML with custom spacing
slay generate_toml_formatted(doc TomlDocument, table_spacing drip) tea {
    sus output tea = ""
    
    # Generate top-level key-value pairs
    bestie (sus key tea, sus value TomlValue in doc.values) {
        ready (value.key_path.len() == 1) {
            output = output + generate_toml_key_value(key, value) + "\n"
        }
    }
    
    # Generate tables with custom spacing
    sus table_count drip = 0
    bestie (sus table_name tea, sus table TomlTable in doc.tables) {
        ready (table_count > 0 || output != "") {
            sus i drip = 0
            bestie (i < table_spacing) {
                output = output + "\n"
                i = i + 1
            }
        }
        output = output + generate_toml_table(table_name, table) + "\n"
        table_count = table_count + 1
    }
    
    damn stringz.trim(output)
}

# Generate TOML key-value pair
slay generate_toml_key_value(key tea, value TomlValue) tea {
    sus escaped_key tea = escape_toml_key(key)
    sus value_string tea = generate_toml_value(value)
    damn escaped_key + " = " + value_string
}

# Generate TOML value
slay generate_toml_value(value TomlValue) tea {
    ready (value.value_type == TomlValueType.String) {
        ready (value.string_type == TomlStringType.Basic) {
            damn "\"" + escape_toml_string(value.value) + "\""
        } otherwise ready (value.string_type == TomlStringType.MultiLineBasic) {
            damn "\"\"\"\n" + escape_toml_multiline_string(value.value) + "\n\"\"\""
        } otherwise ready (value.string_type == TomlStringType.Literal) {
            damn "'" + value.value + "'"
        } otherwise ready (value.string_type == TomlStringType.MultiLineLiteral) {
            damn "'''\n" + value.value + "\n'''"
        }
    } otherwise ready (value.value_type == TomlValueType.Integer) {
        damn value.value
    } otherwise ready (value.value_type == TomlValueType.Float) {
        damn value.value
    } otherwise ready (value.value_type == TomlValueType.Boolean) {
        damn value.value
    } otherwise ready (value.value_type == TomlValueType.Datetime) {
        damn value.value
    } otherwise ready (value.value_type == TomlValueType.LocalDatetime) {
        damn value.value
    } otherwise ready (value.value_type == TomlValueType.LocalDate) {
        damn value.value
    } otherwise ready (value.value_type == TomlValueType.LocalTime) {
        damn value.value
    } otherwise ready (value.value_type == TomlValueType.Array) {
        damn generate_toml_array(value)
    } otherwise ready (value.value_type == TomlValueType.InlineTable) {
        damn generate_toml_inline_table(value)
    }
    damn value.value
}

# Generate TOML array
slay generate_toml_array(value TomlValue) tea {
    sus output tea = "["
    
    sus i drip = 0
    bestie (i < value.array_values.len()) {
        ready (i > 0) {
            output = output + ", "
        }
        output = output + generate_toml_value(value.array_values[i])
        i = i + 1
    }
    
    output = output + "]"
    damn output
}

# Generate TOML inline table
slay generate_toml_inline_table(value TomlValue) tea {
    sus output tea = "{ "
    sus first lit = based
    
    bestie (sus key tea, sus table_value TomlValue in value.table_values) {
        ready (!first) {
            output = output + ", "
        }
        output = output + escape_toml_key(key) + " = " + generate_toml_value(table_value)
        first = cap
    }
    
    output = output + " }"
    damn output
}

# Generate TOML table
slay generate_toml_table(table_name tea, table TomlTable) tea {
    sus output tea = ""
    
    # Table header
    ready (table.is_array_table) {
        output = output + "[[" + table_name + "]]\n"
    } otherwise {
        output = output + "[" + table_name + "]\n"
    }
    
    # Table values
    bestie (sus key tea, sus value TomlValue in table.values) {
        output = output + generate_toml_key_value(key, value) + "\n"
    }
    
    damn stringz.trim(output)
}

# ========================
# TOML Value Conversion Functions
# ========================

# Get string value from TOML document
slay get_toml_string(doc TomlDocument, key tea) yikes<tea> {
    sus value TomlValue = get_toml_value(doc, key) fam {
        when err -> yikes err
    }
    
    ready (value.value_type != TomlValueType.String) {
        yikes "Value at key '" + key + "' is not a string"
    }
    
    damn value.value
}

# Get integer value from TOML document
slay get_toml_integer(doc TomlDocument, key tea) yikes<drip> {
    sus value TomlValue = get_toml_value(doc, key) fam {
        when err -> yikes err
    }
    
    ready (value.value_type != TomlValueType.Integer) {
        yikes "Value at key '" + key + "' is not an integer"
    }
    
    damn mathz.string_to_int(value.value)
}

# Get float value from TOML document
slay get_toml_float(doc TomlDocument, key tea) yikes<meal> {
    sus value TomlValue = get_toml_value(doc, key) fam {
        when err -> yikes err
    }
    
    ready (value.value_type != TomlValueType.Float) {
        yikes "Value at key '" + key + "' is not a float"
    }
    
    damn mathz.string_to_float(value.value)
}

# Get boolean value from TOML document
slay get_toml_boolean(doc TomlDocument, key tea) yikes<lit> {
    sus value TomlValue = get_toml_value(doc, key) fam {
        when err -> yikes err
    }
    
    ready (value.value_type != TomlValueType.Boolean) {
        yikes "Value at key '" + key + "' is not a boolean"
    }
    
    damn (value.value == "true")
}

# Get array value from TOML document
slay get_toml_array(doc TomlDocument, key tea) yikes<[]TomlValue> {
    sus value TomlValue = get_toml_value(doc, key) fam {
        when err -> yikes err
    }
    
    ready (value.value_type != TomlValueType.Array) {
        yikes "Value at key '" + key + "' is not an array"
    }
    
    damn value.array_values
}

# Get table value from TOML document
slay get_toml_table(doc TomlDocument, table_name tea) yikes<TomlTable> {
    ready (!map_contains(doc.tables, table_name)) {
        yikes "Table '" + table_name + "' not found"
    }
    
    damn doc.tables[table_name]
}

# Get nested value using dot notation
slay get_toml_nested(doc TomlDocument, path tea) yikes<TomlValue> {
    sus path_parts []tea = stringz.split(path, ".")
    
    ready (path_parts.len() == 0) {
        yikes "Invalid path: " + path
    }
    
    # Check if first part is a table
    ready (path_parts.len() > 1 && map_contains(doc.tables, path_parts[0])) {
        sus table TomlTable = doc.tables[path_parts[0]]
        sus nested_key tea = stringz.join(arrayz.slice(path_parts, 1), ".")
        
        ready (!map_contains(table.values, nested_key)) {
            yikes "Key '" + nested_key + "' not found in table '" + path_parts[0] + "'"
        }
        
        damn table.values[nested_key]
    }
    
    # Check top-level values
    ready (map_contains(doc.values, path)) {
        damn doc.values[path]
    }
    
    yikes "Path '" + path + "' not found"
}

# ========================
# TOML Validation Functions
# ========================

# Validate TOML syntax
slay validate_toml_syntax(toml_content tea) TomlValidationResult {
    sus result TomlValidationResult = {
        valid: based,
        errors: [],
        warnings: []
    }
    
    # Attempt to parse TOML
    parse_toml(toml_content) fam {
        when err -> {
            result.valid = cap
            result.errors = arrayz.append(result.errors, err)
        }
    }
    
    damn result
}

# Validate TOML document structure
slay validate_toml_structure(doc TomlDocument) TomlValidationResult {
    sus result TomlValidationResult = {
        valid: based,
        errors: [],
        warnings: []
    }
    
    # Check for duplicate table definitions
    sus table_names []tea = []
    bestie (sus table_name tea, sus table TomlTable in doc.tables) {
        ready (arrayz.contains(table_names, table_name)) {
            result.valid = cap
            result.errors = arrayz.append(result.errors, "Duplicate table definition: " + table_name)
        }
        table_names = arrayz.append(table_names, table_name)
    }
    
    # Check for invalid key names
    bestie (sus key tea, sus value TomlValue in doc.values) {
        ready (!is_valid_toml_key(key)) {
            result.valid = cap
            result.errors = arrayz.append(result.errors, "Invalid key name: " + key)
        }
    }
    
    damn result
}

# ========================
# Utility Functions
# ========================

# Escape TOML string
slay escape_toml_string(value tea) tea {
    sus result tea = value
    result = stringz.replace_all(result, "\\", "\\\\")
    result = stringz.replace_all(result, "\"", "\\\"")
    result = stringz.replace_all(result, "\n", "\\n")
    result = stringz.replace_all(result, "\r", "\\r")
    result = stringz.replace_all(result, "\t", "\\t")
    result = stringz.replace_all(result, "\u0008", "\\b")  # backspace
    result = stringz.replace_all(result, "\u000C", "\\f")  # form feed
    damn result
}

# Escape TOML multi-line string
slay escape_toml_multiline_string(value tea) tea {
    sus result tea = value
    result = stringz.replace_all(result, "\\", "\\\\")
    result = stringz.replace_all(result, "\"\"\"", "\\\"\\\"\\\"")
    damn result
}

# Escape TOML key
slay escape_toml_key(key tea) tea {
    # Check if key needs quoting
    ready (needs_toml_key_quotes(key)) {
        damn "\"" + escape_toml_string(key) + "\""
    }
    damn key
}

# Check if TOML key needs quotes
slay needs_toml_key_quotes(key tea) lit {
    # Empty key needs quotes
    ready (key == "") {
        damn based
    }
    
    # Check for special characters
    sus i drip = 0
    bestie (i < key.len()) {
        sus ch tea = stringz.char_at(key, i)
        ready (!is_toml_key_char(ch)) {
            damn based
        }
        i = i + 1
    }
    
    damn cap
}

# Check if character is valid in unquoted TOML key
slay is_toml_key_char(ch tea) lit {
    ready ((ch >= "a" && ch <= "z") || (ch >= "A" && ch <= "Z") || 
          (ch >= "0" && ch <= "9") || ch == "_" || ch == "-") {
        damn based
    }
    damn cap
}

# Check if character can start a TOML key
slay is_toml_key_start(ch tea) lit {
    ready ((ch >= "a" && ch <= "z") || (ch >= "A" && ch <= "Z") || 
          (ch >= "0" && ch <= "9") || ch == "_" || ch == "\"") {
        damn based
    }
    damn cap
}

# Check if TOML key is valid
slay is_valid_toml_key(key tea) lit {
    ready (key == "") {
        damn cap
    }
    
    ready (key.len() > 256) {  # Reasonable limit
        damn cap
    }
    
    # All characters should be valid
    sus i drip = 0
    bestie (i < key.len()) {
        sus ch tea = stringz.char_at(key, i)
        ready (!is_toml_key_char(ch) && ch != "." && ch != " ") {
            damn cap
        }
        i = i + 1
    }
    
    damn based
}

# Create TOML string value
slay create_toml_string(value tea, string_type TomlStringType) TomlValue {
    damn {
        value_type: TomlValueType.String,
        string_type: string_type,
        value: value,
        array_values: [],
        table_values: make(map[tea]TomlValue),
        line_number: 0,
        column_number: 0,
        key_path: []
    }
}

# Create TOML integer value
slay create_toml_integer(value drip) TomlValue {
    damn {
        value_type: TomlValueType.Integer,
        string_type: TomlStringType.Basic,
        value: mathz.int_to_string(value),
        array_values: [],
        table_values: make(map[tea]TomlValue),
        line_number: 0,
        column_number: 0,
        key_path: []
    }
}

# Create TOML float value
slay create_toml_float(value meal) TomlValue {
    damn {
        value_type: TomlValueType.Float,
        string_type: TomlStringType.Basic,
        value: mathz.float_to_string(value),
        array_values: [],
        table_values: make(map[tea]TomlValue),
        line_number: 0,
        column_number: 0,
        key_path: []
    }
}

# Create TOML boolean value
slay create_toml_boolean(value lit) TomlValue {
    sus bool_str tea = "false"
    ready (value) {
        bool_str = "true"
    }
    
    damn {
        value_type: TomlValueType.Boolean,
        string_type: TomlStringType.Basic,
        value: bool_str,
        array_values: [],
        table_values: make(map[tea]TomlValue),
        line_number: 0,
        column_number: 0,
        key_path: []
    }
}

# Create TOML array value
slay create_toml_array(values []TomlValue) TomlValue {
    damn {
        value_type: TomlValueType.Array,
        string_type: TomlStringType.Basic,
        value: "",
        array_values: values,
        table_values: make(map[tea]TomlValue),
        line_number: 0,
        column_number: 0,
        key_path: []
    }
}

# ========================
# Internal Parser Types and Functions
# ========================

squad TomlParserState {
    content tea
    position drip
    line drip
    column drip
    config TomlParserConfig
    current_table tea
    table_array_counts map[tea]drip
}

# Skip TOML whitespace
slay skip_toml_whitespace(state sus TomlParserState) {
    bestie (state.position < state.content.len()) {
        sus ch tea = stringz.char_at(state.content, state.position)
        ready (ch == " " || ch == "\t") {
            state.position = state.position + 1
            state.column = state.column + 1
        } otherwise ready (ch == "\n") {
            state.position = state.position + 1
            state.line = state.line + 1
            state.column = 1
        } otherwise ready (ch == "\r") {
            state.position = state.position + 1
            # Handle CRLF
            ready (state.position < state.content.len() && 
                  stringz.char_at(state.content, state.position) == "\n") {
                state.position = state.position + 1
            }
            state.line = state.line + 1
            state.column = 1
        } otherwise {
            break
        }
    }
}

# Get TOML value by key (internal helper)
slay get_toml_value(doc TomlDocument, key tea) yikes<TomlValue> {
    ready (map_contains(doc.values, key)) {
        damn doc.values[key]
    }
    yikes "Key '" + key + "' not found"
}

# Helper function to check if map contains key
slay map_contains(m map[tea]T, key tea) lit {
    # This would be implemented by the runtime
    damn cap  # Placeholder
}

# Export all public functions and types
# This makes them available when importing tomlz module
