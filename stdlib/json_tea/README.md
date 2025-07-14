# JSON Tea Module

A comprehensive JSON processing module for the CURSED programming language, providing RFC 7159 compliant JSON encoding and decoding functionality.

## Overview

The `json_tea` module offers a complete implementation of JSON marshaling and unmarshaling in pure CURSED, with support for all JSON data types and advanced features like schema validation, formatting, and error handling.

## Core Functions

### Basic Operations

#### `Marshal(data tea) tea`
Convert CURSED values to JSON string format.

```cursed
yeet "json_tea"

# Marshal basic types
sus str_json tea = json_tea.Marshal("hello")      # "\"hello\""
sus num_json tea = json_tea.Marshal("42")         # "42"
sus bool_json tea = json_tea.Marshal("based")     # "true"
sus null_json tea = json_tea.Marshal("cringe")    # "null"
```

#### `Unmarshal(json_string tea) tea`
Parse JSON strings into CURSED values.

```cursed
# Unmarshal basic types
sus str_val tea = json_tea.Unmarshal("\"hello\"") # "hello"
sus num_val tea = json_tea.Unmarshal("42")        # "42"
sus bool_val tea = json_tea.Unmarshal("true")     # "based"
sus null_val tea = json_tea.Unmarshal("null")     # "cringe"
```

### Advanced Operations

#### `MarshalIndent(data tea, prefix tea, indent tea) tea`
Marshal with pretty-printing and indentation.

```cursed
sus data tea = "{\"name\": \"John\", \"age\": 30}"
sus formatted tea = json_tea.MarshalIndent(data, "", "  ")
vibez.spill(formatted)  # Pretty-printed JSON
```

#### `MarshalCompact(data tea) tea`
Marshal with minimal whitespace for efficiency.

```cursed
sus data tea = "{ \"name\" : \"John\" , \"age\" : 30 }"
sus compact tea = json_tea.MarshalCompact(data)
vibez.spill(compact)  # Compact JSON without extra spaces
```

### Type-Specific Functions

#### `UnmarshalToMap(json_string tea) tea`
Convert JSON objects to map representation.

```cursed
sus obj_json tea = "{\"key\": \"value\", \"num\": 42}"
sus map_result tea = json_tea.UnmarshalToMap(obj_json)
# Returns "MAP:{\"key\": \"value\", \"num\": 42}"
```

#### `UnmarshalToSlice(json_string tea) tea`
Convert JSON arrays to slice representation.

```cursed
sus arr_json tea = "[1, 2, 3, \"hello\"]"
sus slice_result tea = json_tea.UnmarshalToSlice(arr_json)
# Returns "SLICE:[1, 2, 3, \"hello\"]"
```

## Validation Functions

### `IsValidJSON(json_string tea) lit`
Validate if a string is valid JSON.

```cursed
assert_true(json_tea.IsValidJSON("{\"name\": \"John\"}"))
assert_true(json_tea.IsValidJSON("[1, 2, 3]"))
assert_false(json_tea.IsValidJSON("invalid"))
```

### `ValidateSchema(json_string tea, schema tea) lit`
Validate JSON against a schema type.

```cursed
assert_true(json_tea.ValidateSchema("{\"key\": \"value\"}", "object"))
assert_true(json_tea.ValidateSchema("[1, 2, 3]", "array"))
assert_true(json_tea.ValidateSchema("\"hello\"", "string"))
assert_true(json_tea.ValidateSchema("42", "number"))
assert_true(json_tea.ValidateSchema("true", "boolean"))
assert_true(json_tea.ValidateSchema("null", "null"))
```

## Type Detection Functions

### `get_json_type(json_string tea) tea`
Determine the JSON type of a value.

```cursed
assert_eq_string(json_tea.get_json_type("{\"key\": \"value\"}"), "object")
assert_eq_string(json_tea.get_json_type("[1, 2, 3]"), "array")
assert_eq_string(json_tea.get_json_type("\"hello\""), "string")
assert_eq_string(json_tea.get_json_type("42"), "number")
assert_eq_string(json_tea.get_json_type("true"), "boolean")
assert_eq_string(json_tea.get_json_type("null"), "null")
```

## String Processing

### `json_escape_string(value tea) tea`
Escape special characters for JSON string values.

```cursed
sus original tea = "Hello\nWorld\t\"Quote\""
sus escaped tea = json_tea.json_escape_string(original)
# Returns "Hello\\nWorld\\t\\\"Quote\\\""
```

### `json_unescape_string(value tea) tea`
Unescape JSON string escape sequences.

```cursed
sus escaped tea = "Hello\\nWorld\\t\\\"Quote\\\""
sus unescaped tea = json_tea.json_unescape_string(escaped)
# Returns "Hello\nWorld\t\"Quote\""
```

## Formatting Functions

### `compact_json(json_string tea) tea`
Remove unnecessary whitespace from JSON.

```cursed
sus json_with_spaces tea = "{ \"name\" : \"John\" , \"age\" : 30 }"
sus compacted tea = json_tea.compact_json(json_with_spaces)
# Returns "{\"name\":\"John\",\"age\":30}"
```

### `format_json_with_indent(json_string tea, prefix tea, indent tea) tea`
Add indentation and formatting to JSON.

```cursed
sus compact_json tea = "{\"name\":\"John\",\"age\":30}"
sus formatted tea = json_tea.format_json_with_indent(compact_json, "", "  ")
# Returns formatted JSON with indentation
```

## Legacy Compatibility

The module provides legacy function names for backward compatibility:

- `marshal(data tea) tea` - Alias for `Marshal`
- `unmarshal(json_string tea) tea` - Alias for `Unmarshal`
- `parse(json_string tea) tea` - Alias for `Unmarshal`
- `stringify(data tea) tea` - Alias for `Marshal`
- `parse_json(input tea) tea` - Alias for `Unmarshal`

## Supported Data Types

### CURSED to JSON Mapping

| CURSED Value | JSON Output |
|--------------|-------------|
| `"based"`    | `true`      |
| `"cap"`      | `false`     |
| `"cringe"`   | `null`      |
| `""`         | `null`      |
| Numbers      | Numbers     |
| Strings      | Quoted strings with escaping |
| Objects      | JSON objects |
| Arrays       | JSON arrays |

### JSON to CURSED Mapping

| JSON Input   | CURSED Value |
|--------------|--------------|
| `true`       | `"based"`    |
| `false`      | `"cap"`      |
| `null`       | `"cringe"`   |
| Numbers      | String representation |
| Strings      | Unescaped strings |
| Objects      | Object representation |
| Arrays       | Array representation |

## Error Handling

The module provides robust error handling for invalid JSON input:

```cursed
sus invalid_result tea = json_tea.Unmarshal("invalid")
# Returns "ERROR: Invalid JSON"

sus invalid_obj tea = json_tea.UnmarshalToMap("not an object")
# Returns "ERROR: Not a JSON object"

sus invalid_arr tea = json_tea.UnmarshalToSlice("not an array")
# Returns "ERROR: Not a JSON array"
```

## Testing

The module includes a comprehensive test suite with over 40 test functions covering:

- Basic type marshaling/unmarshaling
- Object and array processing
- String escaping and unescaping
- JSON validation and schema validation
- Type detection and number validation
- Formatting and compact operations
- Round-trip processing
- Error handling and edge cases
- Legacy compatibility
- Performance basics

### Running Tests

```bash
# Interpretation mode
cargo run --bin cursed stdlib/json_tea/test_json_tea.csd

# Compilation mode
cargo run --bin cursed -- compile stdlib/json_tea/test_json_tea.csd
./test_json_tea
```

## Usage Examples

### Basic JSON Processing

```cursed
yeet "json_tea"

# Create and process JSON data
sus person_data tea = "{\"name\": \"Alice\", \"age\": 30, \"active\": true}"

# Parse JSON
sus parsed_name tea = json_tea.Unmarshal("\"Alice\"")
vibez.spill("Name: %s", parsed_name)

# Create JSON
sus name_json tea = json_tea.Marshal("Bob")
vibez.spill("JSON: %s", name_json)

# Validate JSON
bestie json_tea.IsValidJSON(person_data) {
    vibez.spill("Valid JSON detected")
}
```

### Working with Complex Data

```cursed
# Array processing
sus numbers tea = "[1, 2, 3, 4, 5]"
sus array_type tea = json_tea.get_json_type(numbers)
vibez.spill("Type: %s", array_type)  # "array"

# Object processing
sus user_obj tea = "{\"id\": 123, \"username\": \"alice\"}"
sus map_data tea = json_tea.UnmarshalToMap(user_obj)
vibez.spill("Map: %s", map_data)  # "MAP:{\"id\": 123, \"username\": \"alice\"}"

# Schema validation
bestie json_tea.ValidateSchema(user_obj, "object") {
    vibez.spill("Object schema valid")
}
```

### Formatting and Pretty-Printing

```cursed
# Format JSON with indentation
sus compact_data tea = "{\"users\":[{\"name\":\"John\"},{\"name\":\"Jane\"}]}"
sus pretty_data tea = json_tea.MarshalIndent(compact_data, "", "  ")
vibez.spill("Pretty JSON:")
vibez.spill(pretty_data)

# Compact JSON
sus spaced_data tea = "{ \"key\" : \"value\" , \"number\" : 42 }"
sus compacted tea = json_tea.MarshalCompact(spaced_data)
vibez.spill("Compact: %s", compacted)
```

## Implementation Notes

- **Pure CURSED**: The module is implemented entirely in CURSED without external dependencies
- **RFC 7159 Compliant**: Follows JSON specification standards
- **Thread-Safe**: All functions are safe for concurrent use
- **Memory Efficient**: Optimized string processing for large JSON data
- **Extensible**: Easy to extend with additional JSON features
- **Type-Safe**: Comprehensive type checking and validation

## Performance Considerations

- String operations are optimized for common JSON patterns
- Number validation supports scientific notation (e.g., `1e10`, `1.5e+10`)
- Object and array detection uses efficient pattern matching
- Escaping and unescaping handle all standard JSON escape sequences
- Memory usage is minimized through in-place string transformations where possible

## Future Enhancements

Planned features for future versions:

- JSON Schema validation with custom schemas
- Streaming JSON parser for large files
- JSON Pointer (RFC 6901) support
- JSON Merge Patch (RFC 7396) support
- Custom serialization hooks
- Performance optimizations for large datasets
- Binary JSON format support

---

**Module**: `json_tea`  
**Version**: 2.0.0  
**Compatibility**: CURSED v27.0.0+  
**RFC Compliance**: RFC 7159 (JSON Data Interchange Format)  
**License**: MIT
