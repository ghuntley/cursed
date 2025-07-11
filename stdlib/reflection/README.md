# CURSED Reflection System

A comprehensive reflection system for the CURSED programming language, implemented in pure CURSED. This module provides runtime type inspection, dynamic method calling, and metadata access capabilities for enterprise-grade applications.

## Features

### Core Reflection Capabilities
- **Runtime Type Inspection**: Get detailed information about types at runtime
- **Dynamic Method Calling**: Invoke methods dynamically by name
- **Metadata Access**: Access method signatures, field information, and type metadata
- **Type Conversion**: Convert between different types dynamically
- **Interface Implementation Checking**: Verify if types implement specific interfaces
- **Deep Equality**: Compare values of any type for deep equality
- **Zero Value Generation**: Generate zero values for any type

### Type System Integration
- **Type Information Structure**: Comprehensive type metadata including name, size, kind, methods, and fields
- **Method Information**: Access method signatures, return types, and parameter information
- **Field Information**: Get field names, types, accessibility, and mutability
- **Reflection Values**: Wrapper objects that combine values with their type information

## API Reference

### Core Functions

#### `get_type_info(value interface{}) TypeInfo`
Extract complete type information from any value.

```cursed
sus int_val normie = 42
sus type_info TypeInfo = get_type_info(int_val)
vibez.spill(type_info.name) # Output: "normie"
vibez.spill(type_info.kind) # Output: "integer"
```

#### `reflect_value_of(value interface{}) ReflectValue`
Create a reflection wrapper for any value.

```cursed
sus str_val tea = "hello"
sus rv ReflectValue = reflect_value_of(str_val)
vibez.spill(get_type_name(rv)) # Output: "tea"
```

#### `call_method(rv ReflectValue, method_name tea, args []interface{}) interface{}`
Dynamically call methods on reflected values.

```cursed
sus int_val normie = 42
sus rv ReflectValue = reflect_value_of(int_val)
sus str_result interface{} = call_method(rv, "string", []interface{}{})
vibez.spill(str_result.(tea)) # Output: "42"
```

### Type Inspection Functions

#### `get_type_name(rv ReflectValue) tea`
Get the name of a reflected type.

#### `get_type_kind(rv ReflectValue) tea`
Get the kind of a reflected type (integer, float, string, boolean, character).

#### `get_type_size(rv ReflectValue) normie`
Get the size in bytes of a reflected type.

#### `is_valid(rv ReflectValue) lit`
Check if a reflection value is valid.

### Type Conversion Functions

#### `convert_to_string(value interface{}) tea`
Convert any value to its string representation.

```cursed
sus int_val normie = 42
sus str_val tea = convert_to_string(int_val)
vibez.spill(str_val) # Output: "42"
```

#### `convert_to_int(value interface{}) normie`
Convert any value to an integer.

#### `convert_to_bool(value interface{}) lit`
Convert any value to a boolean.

#### `convert_to_float(value interface{}) meal`
Convert any value to a float.

### Interface and Method Checking

#### `implements_interface(rv ReflectValue, interface_name tea) lit`
Check if a type implements a specific interface.

```cursed
sus int_val normie = 42
sus rv ReflectValue = reflect_value_of(int_val)
sus is_stringer lit = implements_interface(rv, "Stringer")
vibez.spill(is_stringer) # Output: true
```

#### `has_method(rv ReflectValue, method_name tea) lit`
Check if a type has a specific method.

#### `get_method_names(rv ReflectValue) [tea]`
Get all method names for a type.

#### `get_method_info(rv ReflectValue, method_name tea) MethodInfo`
Get detailed information about a specific method.

### Utility Functions

#### `deep_equal(a interface{}, b interface{}) lit`
Compare two values for deep equality.

```cursed
sus val1 normie = 42
sus val2 normie = 42
sus equal lit = deep_equal(val1, val2)
vibez.spill(equal) # Output: true
```

#### `type_assert(rv ReflectValue, target_type tea) interface{}`
Assert that a value is of a specific type.

#### `is_nil(rv ReflectValue) lit`
Check if a reflected value is nil.

#### `get_zero_value(type_name tea) interface{}`
Get the zero value for a specific type.

```cursed
sus zero_int interface{} = get_zero_value("normie")
vibez.spill(zero_int.(normie)) # Output: 0
```

## Supported Types

### Basic Types
- **normie** (i32): 32-bit signed integer
- **thicc** (i64): 64-bit signed integer
- **smol** (i8): 8-bit signed integer
- **mid** (i16): 16-bit signed integer
- **meal** (f64): 64-bit floating point
- **drip** (f32): 32-bit floating point
- **tea** (string): UTF-8 string
- **lit** (bool): Boolean value
- **sip** (char): Single character

### Supported Interfaces
- **Stringer**: Types that can be converted to strings
- **Numeric**: Types that represent numeric values
- **Comparable**: Types that can be compared for equality

## Usage Examples

### Basic Type Inspection
```cursed
yeet "reflection"

slay inspect_value(value interface{}) lit {
    sus rv ReflectValue = reflect_value_of(value)
    
    vibez.spill("Type Name: " + get_type_name(rv))
    vibez.spill("Type Kind: " + get_type_kind(rv))
    vibez.spill("Type Size: " + convert_to_string(get_type_size(rv)))
    vibez.spill("Is Valid: " + convert_to_string(is_valid(rv)))
    
    damn based
}

# Usage
sus my_int normie = 42
inspect_value(my_int)
```

### Dynamic Method Calling
```cursed
yeet "reflection"

slay dynamic_string_conversion(value interface{}) tea {
    sus rv ReflectValue = reflect_value_of(value)
    sus result interface{} = call_method(rv, "string", []interface{}{})
    damn result.(tea)
}

# Usage
sus int_val normie = 42
sus float_val meal = 3.14
sus bool_val lit = based

vibez.spill(dynamic_string_conversion(int_val))   # "42"
vibez.spill(dynamic_string_conversion(float_val)) # "3.14"
vibez.spill(dynamic_string_conversion(bool_val))  # "true"
```

### Interface Implementation Checking
```cursed
yeet "reflection"

slay check_interfaces(value interface{}) lit {
    sus rv ReflectValue = reflect_value_of(value)
    
    yikes implements_interface(rv, "Stringer") {
        vibez.spill("Type implements Stringer interface")
    }
    
    yikes implements_interface(rv, "Numeric") {
        vibez.spill("Type implements Numeric interface")
    }
    
    yikes implements_interface(rv, "Comparable") {
        vibez.spill("Type implements Comparable interface")
    }
    
    damn based
}

# Usage
sus my_value normie = 42
check_interfaces(my_value)
```

### Deep Equality Comparison
```cursed
yeet "reflection"

slay compare_values(a interface{}, b interface{}) lit {
    yikes deep_equal(a, b) {
        vibez.spill("Values are deeply equal")
    } shook {
        vibez.spill("Values are not equal")
    }
    
    damn based
}

# Usage
sus val1 normie = 42
sus val2 normie = 42
sus val3 tea = "42"

compare_values(val1, val2) # Equal
compare_values(val1, val3) # Not equal (different types)
```

### Generic Value Processing
```cursed
yeet "reflection"

slay process_any_value(value interface{}) lit {
    sus rv ReflectValue = reflect_value_of(value)
    
    ready get_type_kind(rv) {
        case "integer":
            vibez.spill("Processing integer: " + convert_to_string(value))
        case "float":
            vibez.spill("Processing float: " + convert_to_string(value))
        case "string":
            vibez.spill("Processing string: " + convert_to_string(value))
        case "boolean":
            vibez.spill("Processing boolean: " + convert_to_string(value))
        default:
            vibez.spill("Processing unknown type")
    }
    
    damn based
}

# Usage
process_any_value(42)      # Integer
process_any_value(3.14)    # Float
process_any_value("hello") # String
process_any_value(based)   # Boolean
```

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/reflection/test_reflection.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/reflection/test_reflection.csd
./test_reflection

# Run specific reflection tests
cargo run --bin cursed test --filter reflection
```

## Test Coverage

The reflection system includes comprehensive tests for:

- **Type Information Extraction**: 15+ test assertions
- **Dynamic Method Calling**: 12+ test assertions  
- **Type Conversions**: 20+ test assertions
- **Metadata Access**: 10+ test assertions
- **Interface Implementation**: 8+ test assertions
- **Deep Equality**: 15+ test assertions
- **Type Assertion**: 4+ test assertions
- **Nil Checking**: 4+ test assertions
- **Zero Value Generation**: 12+ test assertions
- **String Parsing**: 10+ test assertions

Total: **110+ comprehensive test assertions**

## Architecture

### Pure CURSED Implementation
- **No FFI Dependencies**: Entirely implemented in CURSED without external libraries
- **Type-Safe**: Leverages CURSED's type system for safe reflection operations
- **Performance Optimized**: Efficient implementations suitable for production use
- **Extensible**: Easy to extend with new types and interfaces

### Design Patterns
- **Wrapper Pattern**: ReflectValue wraps values with their type information
- **Strategy Pattern**: Different conversion strategies for different types
- **Factory Pattern**: Type-specific creation of reflection objects
- **Template Method**: Common patterns for type inspection and method calling

## Performance Considerations

### Optimization Strategies
- **Type Switching**: Fast type detection using ready/case statements
- **Lazy Evaluation**: Type information computed only when needed
- **Caching**: Method and field information cached for repeated access
- **Memory Efficient**: Minimal memory overhead for reflection operations

### Best Practices
- **Cache Reflection Values**: Reuse ReflectValue objects when possible
- **Batch Operations**: Group multiple reflection operations together
- **Type Checking**: Verify types before expensive operations
- **Error Handling**: Always check validity of reflection values

## Future Enhancements

### Planned Features
- **Struct Field Access**: Direct field access through reflection
- **Array/Slice Reflection**: Enhanced support for collections
- **Function Reflection**: Reflect on function types and signatures
- **Generic Type Support**: Enhanced generics reflection
- **Performance Profiling**: Built-in performance monitoring

### Extension Points
- **Custom Interfaces**: Define and check custom interfaces
- **Type Decorators**: Add metadata to types
- **Reflection Middleware**: Intercept reflection operations
- **Serialization Integration**: Deep integration with serialization systems

## License

This reflection system is part of the CURSED programming language and follows the same license terms.
