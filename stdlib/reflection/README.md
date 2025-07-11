# Reflection Module

A comprehensive reflection system for CURSED that provides runtime type introspection, dynamic value manipulation, and meta-programming capabilities.

## Features

- **Type Introspection**: Complete runtime type information for all CURSED types
- **Dynamic Value Creation**: Create and manipulate values at runtime
- **Struct Reflection**: Inspect struct fields, methods, and metadata
- **Type Conversion**: Safe type conversion with validation
- **Method Invocation**: Dynamic method calling with argument validation
- **Interface Support**: Interface implementation checking and discovery
- **Array/Slice/Pointer Types**: Full support for composite types
- **Field Tags**: Metadata tags for struct fields (JSON, DB, etc.)

## Core Types

### Type Information
```cursed
be_like TypeInfo squad {
    name tea              // Type name
    kind tea              // Type kind (struct, int, float, etc.)
    size normie           // Type size in bytes
    fields [FieldInfo]    // Struct fields
    methods [MethodInfo]  // Type methods
    interfaces [tea]      // Implemented interfaces
    is_pointer lit        // Is pointer type
    is_array lit          // Is array type
    is_slice lit          // Is slice type
    element_type tea      // Element type for arrays/slices/pointers
}
```

### Field Information
```cursed
be_like FieldInfo squad {
    name tea             // Field name
    type_name tea        // Field type name
    offset normie        // Field offset in struct
    size normie          // Field size in bytes
    is_exported lit      // Is field exported
    tags map[tea]tea     // Field tags
}
```

### Method Information
```cursed
be_like MethodInfo squad {
    name tea                    // Method name
    return_type tea             // Return type
    parameters [ParameterInfo]  // Method parameters
    is_exported lit             // Is method exported
    is_variadic lit             // Is variadic method
}
```

### Reflect Value
```cursed
be_like ReflectValue squad {
    type_info TypeInfo    // Type information
    data tea              // Value data
    is_valid lit          // Is value valid
    is_nil lit            // Is value nil
    is_zero lit           // Is value zero
}
```

## Core Functions

### Type Registry
```cursed
create_type_registry() TypeRegistry
register_type(registry TypeRegistry, type_info TypeInfo) TypeRegistry
get_type_info(registry TypeRegistry, type_name tea) TypeInfo
has_type(registry TypeRegistry, type_name tea) lit
get_all_types(registry TypeRegistry) [TypeInfo]
```

### Type Creation
```cursed
create_type_info(name tea, kind tea, size normie) TypeInfo
create_struct_type(name tea, fields [FieldInfo]) TypeInfo
create_array_type(element_type tea, length normie) TypeInfo
create_slice_type(element_type tea) TypeInfo
create_pointer_type(element_type tea) TypeInfo
```

### Value Operations
```cursed
create_reflect_value(type_info TypeInfo, data tea) ReflectValue
get_value_type(value ReflectValue) TypeInfo
get_value_data(value ReflectValue) tea
is_valid_value(value ReflectValue) lit
is_zero_value_reflect(value ReflectValue) lit
```

### Type Conversion
```cursed
can_convert(from_type TypeInfo, to_type TypeInfo) lit
convert_value(value ReflectValue, target_type TypeInfo) ReflectValue
is_numeric_type(type_info TypeInfo) lit
```

## Usage Examples

### Basic Type Introspection
```cursed
// Create type registry
sus registry TypeRegistry = create_type_registry()

// Get type information
sus int_type TypeInfo = get_type_info(registry, "normie")
vibez.spill("Type: " + int_type.name + ", Size: " + string(int_type.size))

// Check if type exists
vibes has_type(registry, "custom_type") {
    vibez.spill("Custom type found")
}
```

### Struct Reflection
```cursed
// Define struct fields
sus id_field FieldInfo = FieldInfo{
    name: "id",
    type_name: "normie",
    offset: 0,
    size: 4,
    is_exported: based,
    tags: {"json": "id", "db": "user_id"}
}

sus name_field FieldInfo = FieldInfo{
    name: "name",
    type_name: "tea",
    offset: 4,
    size: 16,
    is_exported: based,
    tags: {"json": "name", "db": "username"}
}

// Create struct type
sus fields [FieldInfo] = [id_field, name_field]
sus person_type TypeInfo = create_struct_type("Person", fields)

// Register the type
registry = register_type(registry, person_type)

// Inspect struct
vibez.spill("Struct: " + person_type.name)
vibez.spill("Size: " + string(person_type.size))
vibez.spill("Fields: " + string(get_field_count(person_type)))

// Access fields
vibes has_field(person_type, "id") {
    sus field FieldInfo = get_field_by_name(person_type, "id")
    vibez.spill("Field: " + field.name + " (" + field.type_name + ")")
    vibez.spill("JSON tag: " + get_field_tag(field, "json"))
}
```

### Dynamic Value Creation
```cursed
// Create reflect values
sus int_type TypeInfo = get_type_info(registry, "normie")
sus int_value ReflectValue = create_reflect_value(int_type, "42")

vibes is_valid_value(int_value) {
    vibez.spill("Value is valid: " + get_value_data(int_value))
}

// Check for zero values
sus zero_value ReflectValue = create_reflect_value(int_type, "0")
vibes is_zero_value_reflect(zero_value) {
    vibez.spill("Value is zero")
}
```

### Type Conversion
```cursed
// Check conversion compatibility
sus from_type TypeInfo = get_type_info(registry, "normie")
sus to_type TypeInfo = get_type_info(registry, "meal")

vibes can_convert(from_type, to_type) {
    vibez.spill("Can convert int to float")
    
    // Perform conversion
    sus int_val ReflectValue = create_reflect_value(from_type, "42")
    sus float_val ReflectValue = convert_value(int_val, to_type)
    
    vibes is_valid_value(float_val) {
        vibez.spill("Converted value: " + get_value_data(float_val))
    }
}
```

### Array and Slice Types
```cursed
// Create array type
sus array_type TypeInfo = create_array_type("normie", 10)
vibez.spill("Array type: " + array_type.name)
vibez.spill("Element type: " + get_element_type(array_type))
vibez.spill("Is array: " + string(is_array_type(array_type)))

// Create slice type
sus slice_type TypeInfo = create_slice_type("tea")
vibez.spill("Slice type: " + slice_type.name)
vibez.spill("Is slice: " + string(is_slice_type(slice_type)))

// Create pointer type
sus pointer_type TypeInfo = create_pointer_type("normie")
vibez.spill("Pointer type: " + pointer_type.name)
vibez.spill("Is pointer: " + string(is_pointer_type(pointer_type)))
```

### Interface Support
```cursed
// Create interface type
sus writer_interface TypeInfo = create_type_info("Writer", "interface", 8)
writer_interface = add_interface(writer_interface, "Writer")

// Check interface implementation
vibes implements_interface(writer_interface, "Writer") {
    vibez.spill("Type implements Writer interface")
}

// Get implemented interfaces
sus interfaces [tea] = get_implemented_interfaces(writer_interface)
bestie i := 0; i < len(interfaces); i++ {
    vibez.spill("Implements: " + interfaces[i])
}
```

### Method Reflection
```cursed
// Define method parameters
sus param ParameterInfo = ParameterInfo{
    name: "value",
    type_name: "normie",
    is_pointer: cap
}

// Define method
sus method MethodInfo = MethodInfo{
    name: "set_value",
    return_type: "void",
    parameters: [param],
    is_exported: based,
    is_variadic: cap
}

// Add method to type
person_type.methods = person_type.methods + [method]

// Check method existence
vibes has_method(person_type, "set_value") {
    sus found_method MethodInfo = get_method_by_name(person_type, "set_value")
    vibez.spill("Method: " + found_method.name)
    vibez.spill("Return type: " + found_method.return_type)
    vibez.spill("Parameters: " + string(len(found_method.parameters)))
}
```

### Field Tags
```cursed
// Access field tags
sus user_field FieldInfo = get_field_by_name(person_type, "name")

vibes has_field_tag(user_field, "json") {
    sus json_tag tea = get_field_tag(user_field, "json")
    vibez.spill("JSON field name: " + json_tag)
}

vibes has_field_tag(user_field, "db") {
    sus db_tag tea = get_field_tag(user_field, "db")
    vibez.spill("Database column: " + db_tag)
}
```

## Advanced Features

### Type Comparison
```cursed
// Compare types
sus type1 TypeInfo = get_type_info(registry, "normie")
sus type2 TypeInfo = get_type_info(registry, "normie")

vibes types_equal(type1, type2) {
    vibez.spill("Types are equal")
}

// Check assignability
vibes is_assignable(type1, type2) {
    vibez.spill("Type1 can be assigned to Type2")
}
```

### Type Utilities
```cursed
// Get type size
sus size normie = get_type_size("normie")
vibez.spill("Size of normie: " + string(size))

// Check numeric types
sus float_type TypeInfo = get_type_info(registry, "meal")
vibes is_numeric_type(float_type) {
    vibez.spill("meal is a numeric type")
}
```

## Testing

Run the comprehensive test suite:
```bash
cargo run --bin cursed stdlib/reflection/test_reflection.csd
```

Test both interpretation and compilation modes:
```bash
cargo run --bin cursed stdlib/reflection/test_reflection.csd
cargo run --bin cursed -- compile stdlib/reflection/test_reflection.csd
./test_reflection
```

## Performance Considerations

- **Type Registry**: Efficient type lookup with hash maps
- **Value Creation**: Minimal overhead for reflect value creation
- **Type Conversion**: Optimized conversion paths for common types
- **Field Access**: Fast field lookup by name and index
- **Method Calls**: Efficient method resolution and invocation

## Integration

The reflection module integrates with:
- **Serialization**: Automatic struct serialization using field tags
- **Validation**: Type-safe validation using reflection
- **ORM**: Database mapping using struct field tags
- **JSON**: Automatic JSON marshaling/unmarshaling
- **Templates**: Dynamic template rendering with reflection

## Best Practices

1. **Type Safety**: Always check type compatibility before operations
2. **Performance**: Cache type information for frequently used types
3. **Error Handling**: Validate reflect values before use
4. **Memory**: Clean up reflect values when no longer needed
5. **Security**: Validate field access and method calls
6. **Documentation**: Document struct field tags and their meanings

This reflection system provides powerful meta-programming capabilities while maintaining type safety and performance in CURSED applications.
