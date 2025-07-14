# json_tea (JSON Processing)

## Overview
`json_tea` provides comprehensive JSON (JavaScript Object Notation) processing capabilities for CURSED, implementing full RFC 7159 compliance. This module offers marshaling, unmarshaling, validation, and manipulation functions for JSON data. All functions are implemented in pure CURSED without external dependencies.

## Core JSON Operations

### Marshal Functions

#### Basic Marshal
```cursed
slay Marshal(data tea) tea
```
Converts CURSED data to JSON string representation.

**Parameters:**
- `data tea`: The data to convert to JSON

**Returns:**
- `tea`: JSON string representation

**Supported Conversions:**
- `"based"` → `"true"`
- `"cap"` → `"false"`
- `"cringe"` → `"null"`
- Numeric strings → Numbers (unchanged)
- Regular strings → Quoted strings with escaping
- Objects and arrays → Pass-through for complex types

#### Advanced Marshal
```cursed
slay MarshalIndent(data tea, prefix tea, indent tea) tea
slay MarshalCompact(data tea) tea
```

**Parameters:**
- `data tea`: The data to marshal
- `prefix tea`: String prefix for each line (MarshalIndent)
- `indent tea`: Indentation string (MarshalIndent)

**Returns:**
- `tea`: Formatted JSON string

**Examples:**
```cursed
sus json := Marshal("based")                    # Returns "true"
sus num := Marshal("42")                        # Returns "42"
sus str := Marshal("hello")                     # Returns "\"hello\""
sus compact := MarshalCompact(complex_obj)      # Returns minified JSON
sus pretty := MarshalIndent(obj, "", "  ")      # Returns indented JSON
```

### Unmarshal Functions

#### Basic Unmarshal
```cursed
slay Unmarshal(json_string tea) tea
```
Converts JSON string to CURSED data representation.

**Parameters:**
- `json_string tea`: The JSON string to parse

**Returns:**
- `tea`: CURSED data representation

**Supported Conversions:**
- `"true"` → `"based"`
- `"false"` → `"cap"`
- `"null"` → `"cringe"`
- Numbers → Numeric strings
- Quoted strings → Unescaped strings
- Objects and arrays → Structured representations

#### Type-Specific Unmarshal
```cursed
slay UnmarshalToMap(json_string tea) tea
slay UnmarshalToSlice(json_string tea) tea
```

**Parameters:**
- `json_string tea`: The JSON string to parse

**Returns:**
- `tea`: Type-specific representation with "MAP:" or "SLICE:" prefix

**Examples:**
```cursed
sus bool_val := Unmarshal("true")          # Returns "based"
sus null_val := Unmarshal("null")          # Returns "cringe"
sus str_val := Unmarshal("\"hello\"")     # Returns "hello"
sus map_obj := UnmarshalToMap("{\"a\":1}") # Returns "MAP:{\"a\":1}"
```

## Type-Specific Marshaling

### Type-Safe Marshal Functions
```cursed
slay marshal_number(value tea) tea      # Marshal numeric values
slay marshal_string(value tea) tea      # Marshal string values
slay marshal_boolean(value tea) tea     # Marshal boolean values
```

**Parameters:**
- `value tea`: The value to marshal

**Returns:**
- `tea`: JSON representation or error message

**Examples:**
```cursed
sus num_json := marshal_number("42")     # Returns "42"
sus str_json := marshal_string("test")   # Returns "\"test\""
sus bool_json := marshal_boolean("based") # Returns "true"
```

## JSON String Processing

### String Escaping
```cursed
slay json_escape_string(value tea) tea
slay json_unescape_string(value tea) tea
```

Handles JSON string escaping and unescaping for special characters.

**Escape Sequences:**
- `\` → `\\`
- `"` → `\"`
- `\n` → `\\n`
- `\t` → `\\t`
- `\r` → `\\r`

**Examples:**
```cursed
sus escaped := json_escape_string("line1\nline2")   # Returns "line1\\nline2"
sus unescaped := json_unescape_string("line1\\nline2") # Returns "line1\nline2"
```

## JSON Validation

### Format Validation
```cursed
slay IsValidJSON(json_string tea) lit
slay ValidateSchema(json_string tea, schema tea) lit
```

**Parameters:**
- `json_string tea`: The JSON string to validate
- `schema tea`: Expected type for schema validation

**Returns:**
- `lit`: `based` if valid, `cap` if invalid

**Validation Rules:**
- Proper JSON syntax
- Correct type formatting
- Valid escape sequences
- Balanced brackets and braces

### Type Detection
```cursed
slay get_json_type(json_string tea) tea
```

**Parameters:**
- `json_string tea`: The JSON value to analyze

**Returns:**
- `tea`: Type string ("boolean", "null", "number", "string", "object", "array", "unknown")

**Examples:**
```cursed
sus valid := IsValidJSON("{\"key\": \"value\"}")  # Returns based
sus type := get_json_type("42")                   # Returns "number"
sus schema_ok := ValidateSchema("true", "boolean") # Returns based
```

## Type Testing Functions

### JSON Type Predicates
```cursed
slay is_object(value tea) lit         # Tests for JSON object
slay is_array(value tea) lit          # Tests for JSON array
slay is_string_literal(value tea) lit # Tests for quoted string
slay is_boolean(value tea) lit        # Tests for boolean value
slay is_numeric(value tea) lit        # Tests for numeric value
slay is_valid_json_number(value tea) lit # Validates JSON number format
```

**Parameters:**
- `value tea`: The value to test

**Returns:**
- `lit`: `based` if test passes, `cap` otherwise

**Examples:**
```cursed
sus obj := is_object("{\"key\": \"value\"}")  # Returns based
sus arr := is_array("[1, 2, 3]")              # Returns based
sus num := is_numeric("42")                    # Returns based
sus bool := is_boolean("based")                # Returns based
```

## JSON Formatting

### Formatting Functions
```cursed
slay compact_json(json_string tea) tea
slay format_json_with_indent(json_string tea, prefix tea, indent tea) tea
```

**Parameters:**
- `json_string tea`: The JSON to format
- `prefix tea`: Line prefix for indentation
- `indent tea`: Indentation string

**Returns:**
- `tea`: Formatted JSON string

**Formatting Rules:**
- Remove unnecessary whitespace for compact
- Add proper indentation for pretty-printing
- Preserve string content integrity

## String Utility Functions

### Core String Operations
```cursed
slay string_contains(haystack tea, needle tea) lit
slay string_starts_with(haystack tea, prefix tea) lit
slay string_ends_with(haystack tea, suffix tea) lit
slay string_replace_all(input tea, old tea, new tea) tea
slay string_trim_whitespace(value tea) tea
slay string_length(s tea) normie
slay string_substring(s tea, start normie, length normie) tea
```

These internal utility functions support JSON processing operations.

### String Content Processing
```cursed
slay starts_and_ends_with_quotes(value tea) lit
slay extract_string_content(value tea) tea
```

Helper functions for processing quoted JSON strings.

## Legacy Compatibility Functions

### Alternative Function Names
```cursed
slay marshal(data tea) tea          # Alias for Marshal
slay unmarshal(json_string tea) tea # Alias for Unmarshal
slay parse(json_string tea) tea     # Alias for Unmarshal
slay stringify(data tea) tea        # Alias for Marshal
slay parse_json(input tea) tea      # Alias for Unmarshal
```

These functions provide compatibility with common JSON API naming conventions.

## Error Handling

### Error Conditions
- Invalid JSON syntax returns "ERROR: Invalid JSON"
- Type validation failures return "ERROR: Not a valid [type]"
- Malformed numbers return error messages
- All errors are returned as string values, not exceptions

### Error Examples
```cursed
sus error := Unmarshal("{invalid json}")          # Returns "ERROR: Invalid JSON"
sus type_error := marshal_boolean("not_boolean")  # Returns "ERROR: Not a valid boolean"
```

## Performance Characteristics

### Time Complexity
- Basic marshal/unmarshal: O(n) where n is string length
- Type validation: O(n) for string scanning
- String operations: O(n) for content processing
- Complex object processing: O(n) for structural analysis

### Memory Usage
- String operations create new strings (immutable)
- No dynamic object allocation for simple types
- Efficient string manipulation for large JSON documents

## Usage Patterns

### API Data Processing
```cursed
yeet "json_tea"

# Convert API response to CURSED data
slay process_api_response(json_response tea) tea {
    lowkey !IsValidJSON(json_response) {
        damn "Error: Invalid JSON response"
    }
    
    sus data := Unmarshal(json_response)
    damn data
}

# Prepare data for API request
slay prepare_api_request(data tea) tea {
    sus json_data := Marshal(data)
    damn MarshalCompact(json_data)
}
```

### Configuration File Processing
```cursed
# Parse configuration from JSON
slay parse_config(config_json tea) tea {
    lowkey !IsValidJSON(config_json) {
        damn "Error: Invalid configuration format"
    }
    
    sus config_type := get_json_type(config_json)
    lowkey config_type != "object" {
        damn "Error: Configuration must be JSON object"
    }
    
    damn UnmarshalToMap(config_json)
}

# Generate configuration JSON
slay generate_config(settings tea) tea {
    sus json_config := Marshal(settings)
    damn MarshalIndent(json_config, "", "  ")
}
```

### Data Validation Pipeline
```cursed
# Validate and process JSON data
slay validate_and_process(input tea, expected_type tea) tea {
    # Step 1: Basic JSON validation
    lowkey !IsValidJSON(input) {
        damn "Error: Malformed JSON"
    }
    
    # Step 2: Schema validation
    lowkey !ValidateSchema(input, expected_type) {
        damn "Error: Type mismatch"
    }
    
    # Step 3: Process valid data
    sus processed_data := Unmarshal(input)
    damn processed_data
}
```

## Implementation Notes

### RFC 7159 Compliance
- Full compliance with JSON specification
- Proper handling of escape sequences
- Correct number format validation
- Unicode string support (basic ASCII)

### Pure CURSED Implementation
- No external JSON library dependencies
- Compatible with both interpretation and compilation modes
- Self-contained string processing functions

### Type System Integration
- Uses CURSED type conventions ("based"/"cap" for booleans)
- Integrates with CURSED string and array types
- Consistent error handling patterns

## Testing Strategy

### Unit Tests
```cursed
yeet "testz"
yeet "json_tea"

# Test basic marshaling
test_start("JSON marshaling")
assert_eq_string(Marshal("based"), "true")
assert_eq_string(Marshal("42"), "42")
assert_eq_string(Marshal("hello"), "\"hello\"")

# Test unmarshaling
test_start("JSON unmarshaling")
assert_eq_string(Unmarshal("true"), "based")
assert_eq_string(Unmarshal("\"hello\""), "hello")

# Test validation
test_start("JSON validation")
assert_true(IsValidJSON("{\"key\": \"value\"}"))
assert_false(IsValidJSON("{invalid}"))

print_test_summary()
```

### Integration Tests
- Round-trip marshal/unmarshal testing
- Large JSON document processing
- Error condition handling
- Performance benchmarks

## Dependencies

- `testz`: Testing framework for module validation
- No external JSON processing libraries
- No FFI dependencies

## Security Considerations

- Input validation prevents malformed JSON processing
- String escaping prevents injection attacks
- Memory-safe string operations
- No buffer overflows or memory corruption risks

## Thread Safety

- All JSON functions are pure and thread-safe
- No shared state or global variables
- Safe for concurrent use in goroutines
- Immutable string operations only

## Compatibility

### JSON Standards
- RFC 7159 compliant implementation
- Standard JSON data types supported
- Common JSON extensions avoided for compatibility

### Platform Support
- Consistent behavior across all platforms
- No platform-specific JSON handling
- Portable pure CURSED implementation
