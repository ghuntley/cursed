# Reflection Module

The `reflect` module provides runtime type information and introspection capabilities. Critical for self-hosting and dynamic behavior.

## Core Functions

### Type Information
- `type_info_int() (normie, tea, normie, []tea)` - Get type info for integers
- `type_info_string() (normie, tea, normie, []tea)` - Get type info for strings
- `type_info_bool() (normie, tea, normie, []tea)` - Get type info for booleans
- `type_info_float() (normie, tea, normie, []tea)` - Get type info for floats
- `type_info_struct(name tea, fields []tea) (normie, tea, normie, []tea)` - Create struct type info
- `type_info_array(element_type tea, size normie) (normie, tea, normie, []tea)` - Create array type info

### Type Checking
- `is_int_type(type_info (normie, tea, normie, []tea)) lit` - Check if type is integer
- `is_string_type(type_info (normie, tea, normie, []tea)) lit` - Check if type is string
- `is_bool_type(type_info (normie, tea, normie, []tea)) lit` - Check if type is boolean
- `is_struct_type(type_info (normie, tea, normie, []tea)) lit` - Check if type is struct
- `is_array_type(type_info (normie, tea, normie, []tea)) lit` - Check if type is array

### Value Operations
- `value_from_int(val normie) ((normie, tea, normie, []tea), tea, lit)` - Create value from integer
- `value_from_string(val tea) ((normie, tea, normie, []tea), tea, lit)` - Create value from string
- `value_from_bool(val lit) ((normie, tea, normie, []tea), tea, lit)` - Create value from boolean
- `is_valid(value ((normie, tea, normie, []tea), tea, lit)) lit` - Check if value is valid

### Type Conversion
- `can_convert(from_type (normie, tea, normie, []tea), to_type (normie, tea, normie, []tea)) lit` - Check if conversion is possible
- `convert_value(value ((normie, tea, normie, []tea), tea, lit), target_type (normie, tea, normie, []tea)) ((normie, tea, normie, []tea), tea, lit)` - Convert value to target type

### Struct Introspection
- `get_struct_field_count(type_info (normie, tea, normie, []tea)) normie` - Get number of fields
- `get_struct_field_name(type_info (normie, tea, normie, []tea), index normie) tea` - Get field name
- `has_struct_field(type_info (normie, tea, normie, []tea), field_name tea) lit` - Check if field exists

### Function Introspection
- `get_func_param_count(type_info (normie, tea, normie, []tea)) normie` - Get parameter count
- `get_func_param_type(type_info (normie, tea, normie, []tea), index normie) tea` - Get parameter type

### Array Introspection
- `get_array_element_type(type_info (normie, tea, normie, []tea)) tea` - Get element type
- `get_array_size(type_info (normie, tea, normie, []tea)) normie` - Get array size

### Type Registration
- `register_type(type_name tea)` - Register custom type
- `is_type_registered(type_name tea) lit` - Check if type is registered
- `get_registered_types() []tea` - Get all registered types

### Debugging
- `type_to_string(type_info (normie, tea, normie, []tea)) tea` - Convert type to string
- `value_to_string(value ((normie, tea, normie, []tea), tea, lit)) tea` - Convert value to string

## Usage Example

```cursed
yeet "reflect"

# Initialize reflection system
reflect.init_reflection()

# Get type information
sus int_type := reflect.type_info_int()
vibez.spill("Integer type: " + reflect.get_type_name(int_type))

# Create and inspect values
sus value := reflect.value_from_int(42)
vibez.spill("Value type: " + reflect.value_type_name(value))
vibez.spill("Value data: " + reflect.get_value_data(value))

# Type conversion
sus string_type := reflect.type_info_string()
bestie reflect.can_convert(int_type, string_type) {
    sus converted := reflect.convert_value(value, string_type)
    vibez.spill("Converted: " + reflect.get_value_data(converted))
}

# Struct introspection
sus fields := []tea{"name", "age"}
sus person_type := reflect.type_info_struct("Person", fields)
vibez.spill("Person has " + core.tea(reflect.get_struct_field_count(person_type)) + " fields")
```

## Testing

```bash
cargo run --bin cursed stdlib/reflect/test_reflect.csd
```

## Status

✅ **Production Ready** - Fully implemented and tested
- Complete type information system
- Value creation and introspection
- Type conversion capabilities
- Struct, array, and function introspection
- Runtime type registration
- Zero FFI dependencies
