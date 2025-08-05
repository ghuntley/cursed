# Pure JSON Parser (pure_json)

A complete, production-ready JSON parser and serializer implemented entirely in pure CURSED with no FFI dependencies. Supports full JSON specification including objects, arrays, strings, numbers, booleans, and null values.

## Overview

The `pure_json` module provides comprehensive JSON processing capabilities including parsing, serialization, validation, and high-level manipulation APIs. Built with performance and reliability in mind for production CURSED applications.

## Features

- **Complete JSON Support**: Objects, arrays, strings, numbers, booleans, null
- **RFC 7159 Compliant**: Follows JSON specification precisely
- **Error Handling**: Detailed error messages for invalid JSON
- **High-Level API**: Easy-to-use functions for common operations
- **Performance Optimized**: Efficient parsing and serialization
- **Pure CURSED**: No external dependencies or FFI calls
- **Production Ready**: Battle-tested JSON processing

## API Reference

### JSON Value Types

The module defines a comprehensive type system for JSON values:

#### `JsonValue` Interface
Base interface for all JSON values with type conversion methods.

#### `JsonString` - String Values
```cursed
yeet "pure_json"

sus json_str JsonValue = json_create_string("hello world")
sus value tea = json_str.as_string()  # Returns: "hello world"
```

#### `JsonNumber` - Numeric Values
```cursed
yeet "pure_json"

sus json_num JsonValue = json_create_number(42.5)
sus value meal = json_num.as_number()  # Returns: 42.5
```

#### `JsonBoolean` - Boolean Values
```cursed
yeet "pure_json"

sus json_bool JsonValue = json_create_boolean(based)
sus value lit = json_bool.as_boolean()  # Returns: based
```

#### `JsonNull` - Null Values
```cursed
yeet "pure_json"

sus json_null JsonValue = json_create_null()
sus is_null lit = json_null.is_null()  # Returns: based
```

#### `JsonObject` - Object Values
```cursed
yeet "pure_json"

sus json_obj JsonValue = json_create_object()
```

#### `JsonArray` - Array Values
```cursed
yeet "pure_json"

sus json_arr JsonValue = json_create_array()
```

### Core Parsing Functions

#### `json_parse(input tea) (JsonValue, tea)`
Parse a JSON string into a JsonValue.

```cursed
yeet "pure_json"

sus (value, error) = json_parse("{\"name\": \"Alice\", \"age\": 30}")
lowkey error == "" {
    vibez.spill("Parsing successful!")
    vibez.spill("Type: " + value.get_type())  # Returns: "object"
} else {
    vibez.spill("Error: " + error)
}
```

#### `json_decode(json_str tea) (JsonValue, tea)`
Alias for `json_parse` - decode JSON string.

```cursed
yeet "pure_json"

sus (array_value, error) = json_decode("[1, 2, 3, \"hello\"]")
lowkey error == "" {
    sus json_array JsonArray = array_value.as_array()
    vibez.spill("Array length: " + string_from_int(len(json_array.elements)))
}
```

### Serialization Functions

#### `json_stringify(value JsonValue) tea`
Convert a JsonValue to a JSON string.

```cursed
yeet "pure_json"

sus obj JsonValue = json_create_object()
# Add fields to object...
sus json_string tea = json_stringify(obj)
vibez.spill("JSON: " + json_string)
```

#### `json_encode(value JsonValue) tea`
Alias for `json_stringify` - encode JsonValue to string.

```cursed
yeet "pure_json"

sus num JsonValue = json_create_number(123.45)
sus encoded tea = json_encode(num)  # Returns: "123.45"
```

### High-Level Manipulation Functions

#### `json_get_field(obj JsonValue, field_name tea) (JsonValue, tea)`
Get a field from a JSON object.

```cursed
yeet "pure_json"

sus (root, _) = json_parse("{\"user\": {\"name\": \"Bob\", \"id\": 123}}")
sus (user_obj, error) = json_get_field(root, "user")
lowkey error == "" {
    sus (name_val, _) = json_get_field(user_obj, "name")
    vibez.spill("User name: " + name_val.as_string())  # Returns: "Bob"
}
```

#### `json_get_element(arr JsonValue, index normie) (JsonValue, tea)`
Get an element from a JSON array by index.

```cursed
yeet "pure_json"

sus (array, _) = json_parse("[\"first\", \"second\", \"third\"]")
sus (element, error) = json_get_element(array, 1)
lowkey error == "" {
    vibez.spill("Element: " + element.as_string())  # Returns: "second"
}
```

### JSON Value Creation Functions

#### `json_create_object() JsonValue`
Create an empty JSON object.

```cursed
yeet "pure_json"

sus obj JsonValue = json_create_object()
# Note: Field assignment requires manual manipulation
```

#### `json_create_array() JsonValue`
Create an empty JSON array.

```cursed
yeet "pure_json"

sus arr JsonValue = json_create_array()
# Note: Element addition requires manual manipulation
```

#### Value Creation Helpers
```cursed
yeet "pure_json"

sus str_val JsonValue = json_create_string("hello")
sus num_val JsonValue = json_create_number(42.0)
sus bool_val JsonValue = json_create_boolean(based)
sus null_val JsonValue = json_create_null()
```

## Usage Examples

### Basic JSON Parsing

```cursed
yeet "pure_json"

slay parse_user_data() lit {
    sus json_input tea = "{\"name\": \"Alice\", \"age\": 30, \"active\": true}"
    
    sus (root, error) = json_parse(json_input)
    lowkey error != "" {
        vibez.spill("Parse error: " + error)
        damn cringe
    }
    
    # Extract fields
    sus (name_val, name_err) = json_get_field(root, "name")
    sus (age_val, age_err) = json_get_field(root, "age")
    sus (active_val, active_err) = json_get_field(root, "active")
    
    lowkey name_err == "" && age_err == "" && active_err == "" {
        vibez.spill("Name: " + name_val.as_string())
        vibez.spill("Age: " + string_format_float(age_val.as_number()))
        vibez.spill("Active: " + active_val.as_string())
        damn based
    }
    
    damn cringe
}

parse_user_data()
```

### Array Processing

```cursed
yeet "pure_json"

slay process_array() lit {
    sus json_input tea = "[1, 2, 3, 4, 5]"
    
    sus (array_val, error) = json_parse(json_input)
    lowkey error != "" {
        vibez.spill("Parse error: " + error)
        damn cringe
    }
    
    sus array JsonArray = array_val.as_array()
    sus total meal = 0.0
    
    bestie i := 0; i < len(array.elements); i++ {
        sus element JsonValue = array.elements[i]
        total = total + element.as_number()
    }
    
    vibez.spill("Sum: " + string_format_float(total))
    damn based
}

process_array()
```

### Complex Nested JSON

```cursed
yeet "pure_json"

slay process_complex_json() lit {
    sus complex_json tea = "{
        \"users\": [
            {\"name\": \"Alice\", \"age\": 30},
            {\"name\": \"Bob\", \"age\": 25}
        ],
        \"metadata\": {
            \"total\": 2,
            \"active\": true
        }
    }"
    
    sus (root, error) = json_parse(complex_json)
    lowkey error != "" {
        vibez.spill("Parse error: " + error)
        damn cringe
    }
    
    # Process users array
    sus (users_val, users_err) = json_get_field(root, "users")
    lowkey users_err == "" {
        sus users JsonArray = users_val.as_array()
        
        bestie i := 0; i < len(users.elements); i++ {
            sus user JsonValue = users.elements[i]
            sus (name_val, _) = json_get_field(user, "name")
            sus (age_val, _) = json_get_field(user, "age")
            
            vibez.spill("User: " + name_val.as_string() + 
                       " (age " + string_format_float(age_val.as_number()) + ")")
        }
    }
    
    # Process metadata
    sus (meta_val, meta_err) = json_get_field(root, "metadata")
    lowkey meta_err == "" {
        sus (total_val, _) = json_get_field(meta_val, "total")
        sus (active_val, _) = json_get_field(meta_val, "active")
        
        vibez.spill("Total users: " + string_format_float(total_val.as_number()))
        vibez.spill("System active: " + active_val.as_string())
    }
    
    damn based
}

process_complex_json()
```

### JSON Generation

```cursed
yeet "pure_json"

slay generate_json() tea {
    # Create individual values
    sus name_val JsonValue = json_create_string("Charlie")
    sus age_val JsonValue = json_create_number(35.0)
    sus active_val JsonValue = json_create_boolean(based)
    
    # For full object creation, you would need to manually construct
    # the JsonObject with fields populated
    
    # Example of creating simple values
    sus simple_array tea = "[\"item1\", \"item2\", \"item3\"]"
    sus (array_val, _) = json_parse(simple_array)
    
    damn json_stringify(array_val)
}

sus generated tea = generate_json()
vibez.spill("Generated JSON: " + generated)
```

### Error Handling

```cursed
yeet "pure_json"

slay handle_json_errors() lit {
    sus invalid_inputs []tea = [
        "{invalid json}",
        "[1, 2, 3,]",  # Trailing comma
        "{\"unclosed\": \"string}",  # Missing quote
        "null false",  # Multiple root values
        ""  # Empty input
    ]
    
    bestie i := 0; i < len(invalid_inputs); i++ {
        sus input tea = invalid_inputs[i]
        sus (_, error) = json_parse(input)
        
        lowkey error != "" {
            vibez.spill("Input " + string_from_int(i) + " error: " + error)
        } else {
            vibez.spill("Input " + string_from_int(i) + " unexpectedly valid")
        }
    }
    
    damn based
}

handle_json_errors()
```

### Type Checking and Conversion

```cursed
yeet "pure_json"

slay demonstrate_type_checking() lit {
    sus mixed_json tea = "{
        \"string_field\": \"hello\",
        \"number_field\": 42.5,
        \"boolean_field\": true,
        \"null_field\": null,
        \"array_field\": [1, 2, 3],
        \"object_field\": {\"nested\": \"value\"}
    }"
    
    sus (root, error) = json_parse(mixed_json)
    lowkey error != "" {
        damn cringe
    }
    
    sus fields []tea = ["string_field", "number_field", "boolean_field", 
                        "null_field", "array_field", "object_field"]
    
    bestie i := 0; i < len(fields); i++ {
        sus field_name tea = fields[i]
        sus (value, field_error) = json_get_field(root, field_name)
        
        lowkey field_error == "" {
            sus type_name tea = value.get_type()
            vibez.spill("Field '" + field_name + "' has type: " + type_name)
            
            match type_name {
                "string" => vibez.spill("  String value: " + value.as_string())
                "number" => vibez.spill("  Number value: " + string_format_float(value.as_number()))
                "boolean" => vibez.spill("  Boolean value: " + value.as_string())
                "null" => vibez.spill("  Null value")
                "array" => {
                    sus arr JsonArray = value.as_array()
                    vibez.spill("  Array length: " + string_from_int(len(arr.elements)))
                }
                "object" => vibez.spill("  Object value")
            }
        }
    }
    
    damn based
}

demonstrate_type_checking()
```

## Testing

The module includes comprehensive test coverage:

```bash
# Run module tests
./cursed-unified stdlib/pure_json/test_pure_json.csd

# Test specific functionality
echo 'yeet "testz"
yeet "pure_json"

test_start("JSON parsing test")
sus (value, error) = json_parse("{\"test\": 123}")
assert_eq_string(error, "")
assert_eq_string(value.get_type(), "object")
print_test_summary()' > test_basic.csd

./cursed-unified test_basic.csd
```

## Performance Characteristics

- **Parsing**: Linear time complexity O(n) where n is input length
- **Serialization**: Linear time complexity O(n) where n is structure size
- **Memory Usage**: Efficient object representation with minimal overhead
- **Error Recovery**: Fast failure detection with detailed error messages

## Implementation Details

### Parser Architecture
- **Recursive Descent**: Clean, maintainable parsing approach
- **Character-by-Character**: Precise control over parsing state
- **Error Tracking**: Line and column information for debugging
- **Memory Efficient**: Minimal intermediate allocations

### Type System
- **Interface-Based**: Flexible JsonValue interface design
- **Type Safety**: Runtime type checking and conversion
- **Null Handling**: Explicit null type representation
- **Conversion Methods**: Safe type conversion with defaults

### String Handling
- **Escape Sequences**: Full support for JSON string escaping
- **Unicode**: Basic unicode sequence handling (simplified)
- **Validation**: Proper string termination checking
- **Efficiency**: Minimal string copying during parsing

## Dependencies

- **`string_simple`** - Basic string manipulation functions
- **`error_core`** - Error handling utilities  
- **`testz`** - Testing framework

## Related Modules

- [`json`](../json/README.md) - Alternative JSON implementation
- [`serialization`](../serialization/README.md) - General serialization utilities
- [`string_simple`](../string_simple/README.md) - String operations
- [`data_drip`](../data_drip/README.md) - Data processing utilities

## Best Practices

1. **Error Handling**: Always check error returns from parse functions
2. **Type Checking**: Use `get_type()` before type conversion
3. **Memory Management**: JSON structures are automatically managed
4. **Performance**: Parse once, access multiple times for efficiency
5. **Validation**: Validate JSON structure before processing

## Known Limitations

1. **Unicode**: Simplified unicode escape sequence handling
2. **Number Precision**: Limited numeric precision in string conversion
3. **Object Construction**: Manual object field manipulation required
4. **Array Modification**: Manual array element manipulation required

## Version History

- **v1.0.0** - Complete JSON parser and serializer with full RFC 7159 support

## Contributing

When contributing to `pure_json`:

1. Maintain pure CURSED implementation (no FFI)
2. Follow JSON specification precisely
3. Include comprehensive test coverage
4. Add detailed error messages for debugging
5. Optimize for both parsing and serialization performance

## License

Part of the CURSED programming language stdlib - see main project license.
