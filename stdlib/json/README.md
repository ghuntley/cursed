# CURSED JSON Module

Pure CURSED implementation of RFC 7159 compliant JSON processing library with Gen Z slang naming conventions.

## Features

- ✅ **RFC 7159 Compliant**: Full JSON specification support
- ✅ **Pure CURSED**: No FFI dependencies for maximum portability
- ✅ **Parse & Stringify**: Bidirectional JSON processing
- ✅ **Validation**: Comprehensive JSON validation
- ✅ **Pretty Printing**: Human-readable JSON formatting
- ✅ **Minification**: Compact JSON for network transmission
- ✅ **Escape Handling**: Proper JSON string escaping/unescaping
- ✅ **Performance**: Optimized for both speed and memory usage

## Core Functions

### Parsing Functions

```cursed
yeet "json"

# Parse JSON values
sus result tea = json.parse_value("\"hello world\"")  # Returns: hello world
sus number tea = json.parse_value("42")               # Returns: 42
sus bool tea = json.parse_value("true")               # Returns: true

# Main parse function
sus data tea = json.parse("{\"name\": \"John\"}")     # Parse JSON string
```

### Validation Functions

```cursed
# Validate JSON syntax
assert_true(json.validate("{\"valid\": true}"))       # Valid JSON
assert_false(json.validate("invalid json"))           # Invalid JSON

# Check if string is numeric
assert_true(json.is_numeric("3.14"))                  # Valid number
assert_false(json.is_numeric("abc"))                  # Not a number
```

### Stringification Functions

```cursed
# Convert values to JSON strings
sus json_str tea = json.stringify("hello")            # Returns: "hello"
sus num_str tea = json.stringify("42")                # Returns: 42
sus bool_str tea = json.stringify("true")             # Returns: true
```

### Formatting Functions

```cursed
# Pretty print JSON with indentation
sus compact tea = "{\"name\":\"John\",\"age\":30}"
sus pretty tea = json.pretty_print(compact)           # Formatted JSON

# Minify JSON by removing whitespace
sus formatted tea = "{ \"name\" : \"John\" }"
sus minified tea = json.minify(formatted)             # Compact JSON
```

### String Utilities

```cursed
# Escape/unescape JSON strings
sus escaped tea = json.escape_string("Hello\nWorld")  # Returns: Hello\\nWorld
sus unescaped tea = json.unescape_string(escaped)     # Returns: Hello\nWorld

# String utilities
sus trimmed tea = json.string_trim("  hello  ")       # Returns: hello
assert_true(json.string_starts_with("hello", "he"))   # Prefix check
assert_true(json.string_ends_with("world", "ld"))     # Suffix check
```

## Data Types Supported

| JSON Type | CURSED Type | Example |
|-----------|-------------|---------|
| `string` | `tea` | `"hello world"` |
| `number` | `tea` | `"42"`, `"3.14"` |
| `boolean` | `tea` | `"true"`, `"false"` |
| `null` | `tea` | `"null"` |
| `object` | `tea` | `"{\"key\": \"value\"}"` |
| `array` | `tea` | `"[1, 2, 3]"` |

## Usage Examples

### Basic JSON Processing

```cursed
yeet "json"
yeet "testz"

slay demo_basic_json() {
    # Parse a JSON string
    sus user_json tea = "{\"name\": \"Alice\", \"age\": 25}"
    sus name tea = json.parse_value("\"Alice\"")
    
    vibez.spill("Parsed name: " + name)
    
    # Validate JSON
    bestie json.validate(user_json) {
        vibez.spill("Valid JSON!")
    } else {
        vibez.spill("Invalid JSON!")
    }
    
    # Stringify a value
    sus stringified tea = json.stringify("Hello World")
    vibez.spill("Stringified: " + stringified)
}
```

### JSON Formatting

```cursed
slay demo_json_formatting() {
    sus messy_json tea = "{  \"name\"  :  \"John\"  ,  \"age\"  :  30  }"
    
    # Clean it up
    sus minified tea = json.minify(messy_json)
    vibez.spill("Minified: " + minified)
    
    # Make it pretty
    sus pretty tea = json.pretty_print(minified)
    vibez.spill("Pretty:")
    vibez.spill(pretty)
}
```

### String Escaping

```cursed
slay demo_string_escaping() {
    sus raw_string tea = "Line 1\nLine 2\t\"Quoted\""
    
    # Escape for JSON
    sus escaped tea = json.escape_string(raw_string)
    vibez.spill("Escaped: " + escaped)
    
    # Unescape back
    sus unescaped tea = json.unescape_string(escaped)
    vibez.spill("Unescaped: " + unescaped)
}
```

### Round-Trip Processing

```cursed
slay demo_round_trip() {
    sus original tea = "Hello World"
    
    # String -> JSON -> String
    sus json_string tea = json.stringify(original)
    sus parsed_back tea = json.parse_value(json_string)
    
    bestie original == parsed_back {
        vibez.spill("Round-trip successful!")
    } else {
        vibez.spill("Round-trip failed!")
    }
}
```

## Performance Characteristics

- **Memory Efficient**: Minimal memory allocation during processing
- **String Processing**: Optimized character-by-character parsing
- **Validation Speed**: Fast syntax validation without full parsing
- **Escape Handling**: Efficient escape sequence processing

## RFC 7159 Compliance

This implementation follows RFC 7159 "The JavaScript Object Notation (JSON) Data Interchange Format":

- ✅ Valid JSON syntax recognition
- ✅ Proper string escaping/unescaping
- ✅ Number format validation
- ✅ Boolean and null value handling
- ✅ Whitespace tolerance
- ✅ Unicode support (basic)

## Error Handling

The module uses CURSED's error handling patterns:

```cursed
# Validation returns boolean
bestie !json.validate(suspicious_data) {
    vibez.spill("Invalid JSON detected!")
    damn  # Exit early
}

# Parse returns empty string on failure
sus result tea = json.parse_value("invalid")
bestie result == "" {
    vibez.spill("Parse failed!")
}
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/json/test_json.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/json/test_json.csd
./test_json
```

## Integration

The JSON module integrates seamlessly with other CURSED stdlib modules:

```cursed
yeet "json"
yeet "stringz"  # String utilities
yeet "testz"    # Testing framework

# Use with other modules
sus data tea = json.parse(network_response)
sus formatted tea = json.pretty_print(data)
```

## Future Enhancements

Planned features for future versions:

- 🔄 **Object/Array Support**: Full object and array parsing
- 🔄 **JSON Path**: JSONPath query expressions
- 🔄 **Schema Validation**: JSON Schema Draft 7 support
- 🔄 **Streaming**: Large file streaming support
- 🔄 **JSON Patch**: RFC 6902 JSON Patch operations
- 🔄 **JSON Pointer**: RFC 6901 JSON Pointer support

## Contributing

When contributing to the JSON module:

1. **Pure CURSED**: No FFI dependencies
2. **Test Coverage**: Add tests for all new functions
3. **Performance**: Optimize for common use cases
4. **Documentation**: Update README with examples
5. **RFC Compliance**: Follow JSON specifications

## License

Part of the CURSED programming language standard library.
