# CURSED JSON Library

A comprehensive JSON parsing and manipulation library for the CURSED programming language, providing production-ready JSON functionality with full compliance to RFC 7159.

## Overview

The CURSED JSON library provides a complete set of functions for working with JSON data, including parsing, serialization, validation, formatting, and manipulation operations. It's designed to be both powerful and easy to use, following CURSED's idiomatic patterns.

## Features

- **Complete JSON Support**: Parse and generate JSON objects, arrays, strings, numbers, booleans, and null values
- **Validation**: Comprehensive JSON syntax validation with detailed error reporting
- **Formatting**: Pretty-printing with customizable indentation and minification
- **Path Operations**: JSONPath-like access to nested data structures
- **Type Safety**: Strong type checking and conversion functions
- **Error Handling**: Robust error handling with descriptive error messages
- **Unicode Support**: Full Unicode support including escape sequences
- **Performance**: Optimized for both speed and memory efficiency

## Core Functions

### Parsing Functions

```cursed
yeet "json"

// Parse JSON string to map
sus user_data map = json.parse("{\"name\": \"John\", \"age\": 30}")

// Parse JSON array
sus numbers [extra] = json.parse_array("[1, 2, 3, 4, 5]")

// Parse individual JSON values
sus name extra = json.parse_value("\"John Doe\"")
sus age extra = json.parse_value("30")
sus active extra = json.parse_value("true")
```

### Serialization Functions

```cursed
// Convert map to JSON string
sus data map = map_new()
data = map_set(data, "name", "Alice")
data = map_set(data, "age", "25")
sus json_string tea = json.stringify(data)

// Convert array to JSON string
sus numbers [extra] = [1, 2, 3, 4, 5]
sus json_array tea = json.stringify_array(numbers)
```

### Validation Functions

```cursed
// Validate JSON syntax
testz.assert_true(json.validate("{\"name\": \"John\"}"))
testz.assert_false(json.validate("{name: \"John\"}"))  // Missing quotes

// Check if string is valid JSON
bestie json.is_valid_json(user_input) {
    sus parsed_data map = json.parse(user_input)
    // Process valid JSON
} else {
    vibez.spill("Invalid JSON format")
}
```

### Formatting Functions

```cursed
// Pretty print JSON with indentation
sus compact_json tea = "{\"name\":\"John\",\"age\":30}"
sus pretty_json tea = json.pretty_print(compact_json)
vibez.spill(pretty_json)
/* Output:
{
  "name": "John",
  "age": 30
}
*/

// Minify JSON (remove whitespace)
sus minified tea = json.minify(pretty_json)
vibez.spill(minified)  // {"name":"John","age":30}

// Custom indentation
sus custom_indent tea = json.pretty_print_indent(compact_json, 4)
```

### Utility Functions

```cursed
// Safe value access
sus name tea = json.get_value(user_data, "name")
sus email tea = json.get_value_or_default(user_data, "email", "not provided")

// Check for key existence
bestie json.has_key(user_data, "email") {
    vibez.spill("Email: " + json.get_value(user_data, "email"))
}

// Set and remove values
user_data = json.set_value(user_data, "email", "john@example.com")
user_data = json.remove_key(user_data, "temporary_field")

// Get all keys and values
sus keys [tea] = json.get_keys(user_data)
sus values [tea] = json.get_values(user_data)
```

### Array Operations

```cursed
// Array manipulation
sus arr [extra] = json.parse_array("[1, 2, 3]")

// Get and set array values
sus first_value tea = json.get_array_value(arr, 0)
arr = json.set_array_value(arr, 0, "modified")

// Push and pop operations
arr = json.push_array_value(arr, "new_item")
sus last_item tea = json.pop_array_value(arr)

// Array length
sus length normie = json.array_length(arr)
```

### Path Operations

```cursed
// Navigate nested structures using dot notation
sus nested_data map = json.parse("{\"user\": {\"profile\": {\"name\": \"John\"}}}")

// Get nested value
sus name tea = json.get_path(nested_data, "user.profile.name")

// Set nested value
nested_data = json.set_path(nested_data, "user.profile.email", "john@example.com")

// Check if path exists
bestie json.has_path(nested_data, "user.profile.phone") {
    vibez.spill("Phone number exists")
}
```

### Type Checking and Conversion

```cursed
sus value extra = json.parse_value("42")

// Type checking
testz.assert_true(json.is_number(value))
testz.assert_false(json.is_string(value))
testz.assert_eq_string(json.get_type(value), "number")

// Type conversion
sus as_string tea = json.to_string(value)
sus as_number meal = json.to_number(value)
sus as_integer normie = json.to_integer(value)
sus as_boolean lit = json.to_boolean(value)
```

### Merge Operations

```cursed
// Merge objects
sus obj1 map = json.parse("{\"a\": 1, \"b\": 2}")
sus obj2 map = json.parse("{\"c\": 3, \"d\": 4}")
sus merged map = json.merge(obj1, obj2)

// Deep merge (merges nested objects)
sus deep_merged map = json.merge_deep(obj1, obj2)

// Merge arrays
sus arr1 [extra] = json.parse_array("[1, 2, 3]")
sus arr2 [extra] = json.parse_array("[4, 5, 6]")
sus merged_array [extra] = json.merge_arrays(arr1, arr2)
```

### Comparison Functions

```cursed
// Compare JSON values
sus obj1 map = json.parse("{\"name\": \"John\", \"age\": 30}")
sus obj2 map = json.parse("{\"name\": \"John\", \"age\": 30}")
sus obj3 map = json.parse("{\"name\": \"Jane\", \"age\": 25}")

testz.assert_true(json.equals(obj1, obj2))
testz.assert_false(json.equals(obj1, obj3))

// Deep comparison for nested structures
testz.assert_true(json.deep_equals(obj1, obj2))

// Comparison with ordering
sus comparison_result normie = json.compare(obj1, obj3)
```

### Copy Functions

```cursed
// Shallow copy
sus original map = json.parse("{\"name\": \"John\", \"age\": 30}")
sus copied map = json.copy(original)

// Deep copy (copies nested structures)
sus deep_copied map = json.deep_copy(original)
```

## Usage Patterns

### Processing API Responses

```cursed
slay process_api_response(response_json tea) {
    bestie json.validate(response_json) {
        sus data map = json.parse(response_json)
        
        bestie json.has_key(data, "error") {
            vibez.spill("API Error: " + json.get_value(data, "error"))
            damn cap
        }
        
        sus users [extra] = json.to_array(json.get_value(data, "users"))
        bestie json.array_length(users) > 0 {
            vibez.spill("Found " + tea(json.array_length(users)) + " users")
            damn based
        }
    } else {
        vibez.spill("Invalid JSON response")
    }
    
    damn cap
}
```

### Building JSON Responses

```cursed
slay build_user_response(user_id normie, name tea, email tea) tea {
    sus response map = map_new()
    response = json.set_value(response, "id", tea(user_id))
    response = json.set_value(response, "name", name)
    response = json.set_value(response, "email", email)
    response = json.set_value(response, "created_at", time.now_iso8601())
    
    sus metadata map = map_new()
    metadata = json.set_value(metadata, "version", "1.0")
    metadata = json.set_value(metadata, "api", "users")
    
    response = json.set_value(response, "metadata", json.stringify(metadata))
    
    damn json.pretty_print(json.stringify(response))
}
```

### Configuration File Processing

```cursed
slay load_config(config_path tea) map {
    sus config_json tea = io.read_file(config_path)
    
    bestie json.validate(config_json) {
        sus config map = json.parse(config_json)
        
        // Apply defaults
        config = json.get_value_or_default(config, "debug", "false")
        config = json.get_value_or_default(config, "port", "8080")
        config = json.get_value_or_default(config, "host", "localhost")
        
        damn config
    } else {
        vibez.spill("Invalid configuration file format")
        damn map_new()
    }
}
```

## Error Handling

The JSON library provides comprehensive error handling:

```cursed
// Check for parsing errors
bestie json.validate(user_input) {
    sus data map = json.parse(user_input)
    // Process valid JSON
} else {
    sus error_msg tea = json.get_last_error()
    vibez.spill("JSON Error: " + error_msg)
    json.clear_errors()
}

// Check error status
bestie json.has_errors() {
    sus error_msg tea = json.get_last_error()
    vibez.spill("Last error: " + error_msg)
}
```

## Performance Considerations

### Memory Usage

- Use `json.minify()` for network transmission to reduce size
- Use `json.copy()` vs `json.deep_copy()` based on your needs
- Clear large JSON objects when no longer needed

### Parsing Performance

- Validate JSON before parsing if uncertain about source
- Use `json.parse_value()` for single values instead of full parsing
- Consider streaming operations for very large JSON files

### Best Practices

1. **Always validate JSON from external sources**
2. **Use path operations for nested access instead of multiple parse calls**
3. **Prefer `get_value_or_default()` over exception handling**
4. **Use appropriate copy functions (shallow vs deep)**
5. **Clear errors after handling them**

## Common Patterns

### Safe JSON Processing

```cursed
slay safe_json_process(json_string tea) map {
    bestie json.validate(json_string) {
        sus data map = json.parse(json_string)
        
        // Process with defaults
        sus processed map = map_new()
        processed = json.set_value(processed, "id", json.get_value_or_default(data, "id", "unknown"))
        processed = json.set_value(processed, "name", json.get_value_or_default(data, "name", "unnamed"))
        
        damn processed
    } else {
        // Return empty map on error
        damn map_new()
    }
}
```

### JSON Schema Validation

```cursed
slay validate_user_json(user_json tea) lit {
    bestie json.validate(user_json) {
        sus user map = json.parse(user_json)
        
        // Required fields
        bestie !json.has_key(user, "name") || !json.has_key(user, "email") {
            damn cap
        }
        
        // Type validation
        sus name tea = json.get_value(user, "name")
        sus email tea = json.get_value(user, "email")
        
        bestie string_is_empty(name) || string_is_empty(email) {
            damn cap
        }
        
        // Email format validation
        bestie !string_contains(email, "@") {
            damn cap
        }
        
        damn based
    } else {
        damn cap
    }
}
```

## Integration with Other Modules

The JSON library integrates seamlessly with other CURSED stdlib modules:

```cursed
// With HTTP for API calls
sus response tea = http.get("https://api.example.com/users")
sus users [extra] = json.parse_array(response)

// With file I/O
sus config_json tea = io.read_file("config.json")
sus config map = json.parse(config_json)

// With string manipulation
sus formatted_json tea = json.pretty_print(raw_json)
sus lines [tea] = string_split_lines(formatted_json)

// With collections
sus data_map map = json.parse(json_string)
sus keys [tea] = json.get_keys(data_map)
sus sorted_keys [tea] = array_sort(keys)
```

## Security Considerations

1. **Always validate JSON from external sources**
2. **Be careful with deeply nested structures (can cause stack overflow)**
3. **Validate data types after parsing**
4. **Use size limits for large JSON processing**
5. **Sanitize string values if used in SQL or HTML contexts**

## Testing

The JSON library includes comprehensive tests covering:

- Primitive value parsing (strings, numbers, booleans, null)
- Object and array parsing
- Nested structure handling
- Round-trip serialization
- Edge cases and error conditions
- Unicode and escape sequence handling
- Performance with large datasets

Run tests with:
```bash
cargo run --bin cursed stdlib/json/test_json.csd
```

## Contributing

When extending the JSON library:

1. Follow existing naming conventions
2. Add comprehensive tests for new functions
3. Update this documentation
4. Ensure compatibility with existing code
5. Consider performance implications

## License

This module is part of the CURSED programming language standard library.
