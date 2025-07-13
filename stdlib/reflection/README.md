# CURSED Reflection Module

The reflection module provides runtime type inspection and dynamic method invocation capabilities for CURSED programs. This pure CURSED implementation enables enterprise-grade introspection and metaprogramming without FFI dependencies.

## Overview

The reflection system allows programs to examine their own structure at runtime, providing capabilities for:

- Runtime type information extraction
- Dynamic method invocation
- Type conversion utilities
- Deep equality comparison
- Interface implementation checking
- Metadata access for types, methods, and fields

## Core Types

### `TypeInfo`
Represents comprehensive type information:
```cursed
be_like TypeInfo squad {
    name tea        // Type name (e.g., "normie", "lit", "tea")
    size normie     // Size in bytes
    kind tea        // Type category (e.g., "integer", "boolean", "string")
    methods []tea   // Available method names
    fields []tea    // Available field names
}
```

### `ReflectValue`
Wraps any value with reflection capabilities:
```cursed
be_like ReflectValue squad {
    value interface{}    // The actual value
    type_info TypeInfo   // Type metadata
    valid lit           // Whether the reflection value is valid
}
```

### `MethodInfo`
Describes method signatures and accessibility:
```cursed
be_like MethodInfo squad {
    name tea          // Method name
    signature tea     // Full signature string
    return_type tea   // Return type name
    params []tea      // Parameter type names
    accessible lit    // Whether method can be called
}
```

### `FieldInfo`
Provides field metadata:
```cursed
be_like FieldInfo squad {
    name tea         // Field name
    type_name tea    // Field type name
    offset normie    // Byte offset in struct
    accessible lit   // Whether field can be accessed
    mutable lit      // Whether field can be modified
}
```

## Core Functions

### Type Inspection

```cursed
fr fr Get type information for any value
slay get_type_info(value interface{}) TypeInfo

fr fr Create a reflection value wrapper
slay reflect_value_of(value interface{}) ReflectValue

fr fr Check if reflection value is valid
slay is_valid(rv ReflectValue) lit

fr fr Get type name, kind, and size
slay get_type_name(rv ReflectValue) tea
slay get_type_kind(rv ReflectValue) tea
slay get_type_size(rv ReflectValue) normie
```

### Dynamic Method Invocation

```cursed
fr fr Call methods dynamically by name
slay call_method(rv ReflectValue, method_name tea, args []interface{}) interface{}

fr fr Check if type has specific method
slay has_method(rv ReflectValue, method_name tea) lit

fr fr Get all available method names
slay get_method_names(rv ReflectValue) []tea

fr fr Get detailed method information
slay get_method_info(rv ReflectValue, method_name tea) MethodInfo
```

### Type Conversion

```cursed
fr fr Convert values to different types
slay convert_to_string(value interface{}) tea
slay convert_to_int(value interface{}) normie
slay convert_to_bool(value interface{}) lit
slay convert_to_float(value interface{}) meal

fr fr Parse strings to typed values
slay string_to_int(value tea) normie
slay string_to_float(value tea) meal
slay string_to_bool(value tea) lit
```

### Interface and Equality

```cursed
fr fr Check interface implementation
slay implements_interface(rv ReflectValue, interface_name tea) lit

fr fr Deep equality comparison
slay deep_equal(a interface{}, b interface{}) lit

fr fr Type assertion
slay type_assert(rv ReflectValue, target_type tea) interface{}
```

### Utility Functions

```cursed
fr fr Check for nil values
slay is_nil(rv ReflectValue) lit

fr fr Get zero values for types
slay get_zero_value(type_name tea) interface{}

fr fr Create new instances
slay create_instance(type_name tea) interface{}

fr fr Copy reflection values
slay copy_value(rv ReflectValue) ReflectValue

fr fr Type checking utilities
slay is_numeric_type(rv ReflectValue) lit
slay is_comparable_type(rv ReflectValue) lit
slay can_convert_types(from_rv ReflectValue, to_type tea) lit
```

## Usage Examples

### Basic Type Inspection

```cursed
yeet "reflection"

fr fr Inspect a value's type
sus value normie = 42
sus rv ReflectValue = reflect_value_of(value)

vibez.spill("Type name:", get_type_name(rv))     // "interface{}"
vibez.spill("Type kind:", get_type_kind(rv))     // "interface"
vibez.spill("Type size:", get_type_size(rv))     // 8
vibez.spill("Is valid:", is_valid(rv))           // true
```

### Dynamic Method Calling

```cursed
yeet "reflection"

fr fr Call methods dynamically
sus number normie = 42
sus rv ReflectValue = reflect_value_of(number)

fr fr Convert to string dynamically
sus str_result interface{} = call_method(rv, "string", []interface{}{})
vibez.spill("String result:", convert_to_string(str_result))  // "42"

fr fr Check available methods
sus methods []tea = get_method_names(rv)
vibez.spill("Available methods:", methods)  // ["string", "int", "bool", "float"]
```

### Type Conversion

```cursed
yeet "reflection"

fr fr Convert between types
sus bool_val lit = based
vibez.spill("Bool to string:", convert_to_string(bool_val))  // "true"
vibez.spill("Bool to int:", convert_to_int(bool_val))        // 1
vibez.spill("Bool to float:", convert_to_float(bool_val))    // 1.0

fr fr Parse strings
vibez.spill("String to int:", string_to_int("42"))          // 42
vibez.spill("String to bool:", string_to_bool("true"))      // true
vibez.spill("String to float:", string_to_float("3.14"))    // 3.14
```

### Interface Implementation Checking

```cursed
yeet "reflection"

fr fr Check interface implementation
sus value normie = 42
sus rv ReflectValue = reflect_value_of(value)

vibez.spill("Implements Stringer:", implements_interface(rv, "Stringer"))    // true
vibez.spill("Implements Numeric:", implements_interface(rv, "Numeric"))      // true
vibez.spill("Implements Comparable:", implements_interface(rv, "Comparable")) // true
vibez.spill("Has string method:", has_method(rv, "string"))                  // true
```

### Deep Equality and Comparison

```cursed
yeet "reflection"

fr fr Deep equality comparison
sus a normie = 42
sus b normie = 42
sus c normie = 24

vibez.spill("42 == 42:", deep_equal(a, b))  // true
vibez.spill("42 == 24:", deep_equal(a, c))  // false

fr fr Cross-type comparison
sus bool_a lit = based
sus bool_b lit = based
vibez.spill("true == true:", deep_equal(bool_a, bool_b))  // true
```

### Working with Zero Values

```cursed
yeet "reflection"

fr fr Create zero values for types
sus zero_int interface{} = get_zero_value("normie")
sus zero_bool interface{} = get_zero_value("lit")
sus zero_str interface{} = get_zero_value("tea")

vibez.spill("Zero int:", convert_to_int(zero_int))       // 0
vibez.spill("Zero bool:", convert_to_bool(zero_bool))    // false
vibez.spill("Zero string:", convert_to_string(zero_str)) // ""

fr fr Create new instances
sus new_int interface{} = create_instance("normie")
vibez.spill("New int:", convert_to_int(new_int))         // 0
```

### Advanced Metadata Access

```cursed
yeet "reflection"

fr fr Get detailed method information
sus value normie = 42
sus rv ReflectValue = reflect_value_of(value)

sus method_info MethodInfo = get_method_info(rv, "string")
vibez.spill("Method name:", method_info.name)           // "string"
vibez.spill("Method signature:", method_info.signature) // "string() tea"
vibez.spill("Return type:", method_info.return_type)    // "tea"
vibez.spill("Is accessible:", method_info.accessible)   // true

fr fr Check type capabilities
vibez.spill("Is numeric:", is_numeric_type(rv))         // varies by implementation
vibez.spill("Is comparable:", is_comparable_type(rv))   // varies by implementation
vibez.spill("Can convert to string:", can_convert_types(rv, "tea"))  // true
```

### Comprehensive Demo

```cursed
yeet "reflection"

fr fr Run the built-in demonstration
sus demo_success lit = reflection_demo()
vibez.spill("Demo completed:", demo_success)  // true
```

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/reflection/test_reflection.csd
```

Test both interpretation and compilation modes:

```bash
# Interpretation mode
cargo run --bin cursed stdlib/reflection/test_reflection.csd

# Compilation mode
cargo run --bin cursed -- compile stdlib/reflection/test_reflection.csd
./test_reflection
```

## Implementation Notes

### Pure CURSED Implementation
- No FFI dependencies - fully implemented in CURSED language
- Uses simplified type detection suitable for CURSED's type system
- Provides essential reflection capabilities for metaprogramming

### Type System Integration
- Works with CURSED's native types: `normie`, `lit`, `tea`, `meal`, `sip`, etc.
- Handles interface{} values for generic programming
- Supports nil checking and zero value generation

### Performance Considerations
- Optimized for common reflection operations
- Minimal overhead for type inspection
- Efficient method dispatch for known operations

### Supported Interfaces
- **Stringer**: Types that can be converted to strings
- **Numeric**: Types that support numeric operations
- **Comparable**: Types that can be compared for equality

### Limitations
- Simplified type detection due to CURSED's design
- Field access is limited in the current implementation
- Method signatures are simplified for demonstration

## Enterprise Features

### Type Safety
- Robust error handling for invalid operations
- Safe type assertions with nil checking
- Comprehensive validation of reflection operations

### Metaprogramming Support
- Dynamic method invocation for flexible APIs
- Runtime type discovery for generic programming
- Interface implementation checking for polymorphism

### Development Tools
- Comprehensive test coverage with testz framework
- Clear error messages for debugging
- Performance monitoring for reflection-heavy code

## Future Enhancements

- Extended struct field access and modification
- Support for complex types (arrays, slices, maps)
- Performance optimization for repeated operations
- Enhanced method signature parsing
- Generic type parameter support

The reflection module provides a solid foundation for runtime introspection in CURSED applications, enabling sophisticated metaprogramming patterns while maintaining type safety and performance.
