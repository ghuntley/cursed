# CURSED Reflection System

## Overview

The CURSED Reflection System provides comprehensive runtime type information (RTTI) and introspection capabilities for the CURSED programming language. This system enables dynamic programming patterns, runtime type checking, method invocation, and advanced metaprogramming techniques.

## Features

### 🔍 Runtime Type Information (RTTI)
- Complete type metadata for all CURSED types
- Type name, kind, size, and alignment information
- Pointer type detection and reference analysis
- Numeric and comparable type classification

### ⚡ Dynamic Method Calls
- Runtime method invocation by name
- Type-safe method parameter passing
- Return value handling and conversion
- Method existence verification

### 🏗️ Struct Field Inspection
- Complete struct field metadata
- Field type information and byte offsets
- Dynamic field value access and modification
- Field visibility and accessibility checks

### 🔗 Interface Method Discovery
- Interface implementation checking
- Method signature introspection
- Implementation type enumeration
- Dynamic interface satisfaction verification

### 🧬 Generic Type Parameter Inspection
- Type parameter metadata extraction
- Constraint information and validation
- Generic type instantiation tracking
- Parameterized type name generation

### 💾 Memory Layout Introspection
- Byte-level memory layout analysis
- Field offset calculation and padding detection
- Alignment requirement analysis
- Memory efficiency optimization insights

### 🏭 Dynamic Object Creation
- Runtime type instantiation by name
- Zero-value initialization for all types
- Struct creation with field value specification
- Value cloning and deep copy operations

## API Reference

### Core Types

#### TypeInfo
Complete metadata about a type:
```cursed
structure TypeInfo {
    name tea          // Type name (e.g., "normie", "PersonStruct")
    kind tea          // Type kind (e.g., "integer", "struct")
    size normie       // Size in bytes
    align normie      // Alignment requirement
    is_pointer lit    // Whether type is a pointer
    is_comparable lit // Whether type supports comparison
    is_numeric lit    // Whether type is numeric
    methods []MethodInfo // Available methods
    fields []FieldInfo   // Struct fields (empty for non-structs)
}
```

#### MethodInfo
Method metadata for dynamic invocation:
```cursed
structure MethodInfo {
    name tea            // Method name
    signature tea       // Full method signature
    return_type tea     // Return type name
    param_types []tea   // Parameter type names
    is_public lit       // Whether method is publicly accessible
    receiver_type tea   // Type that implements this method
}
```

#### FieldInfo
Struct field metadata:
```cursed
structure FieldInfo {
    name tea        // Field name
    type_name tea   // Field type name
    offset normie   // Byte offset in struct
    size normie     // Field size in bytes
    is_public lit   // Whether field is publicly accessible
    tags []tea      // Field tags for metadata
}
```

#### MemoryLayout
Memory layout information:
```cursed
structure MemoryLayout {
    size normie        // Total size in bytes
    align normie       // Alignment requirement
    field_offsets []normie // Offsets of each field
    padding_bytes normie   // Total padding bytes
}
```

### Basic Type Reflection

#### get_type_info_*()
Get complete type information:
```cursed
sus info TypeInfo = get_type_info_int(42)
vibez.spill(info.name)  // "normie"
vibez.spill(info.kind)  // "integer"
vibez.spill(info.size)  // 4
```

### Dynamic Method Calls

#### call_method_*()
Invoke methods dynamically:
```cursed
sus result tea = call_method_int(42, "to_string", []tea{})
// result = "42"

sus float_result tea = call_method_int(42, "to_float", []tea{})
// float_result = "42.0"
```

### Struct Introspection

#### get_struct_fields_*()
Get struct field metadata:
```cursed
sus fields []FieldInfo = get_struct_fields_person()
for field in fields {
    vibez.spill("Field: " + field.name + " (" + field.type_name + ")")
    vibez.spill("Offset: " + int_to_string_dynamic(field.offset))
    vibez.spill("Size: " + int_to_string_dynamic(field.size))
}
```

#### get_field_value_*() / set_field_value_*()
Dynamic field access:
```cursed
sus person PersonStruct
person.name = "Alice"
person.age = 30

sus name_value tea = get_field_value_person(person, "name")
// name_value = "Alice"

set_field_value_person(&person, "age", "35")
// person.age is now 35
```

### Interface Discovery

#### get_interface_info_*()
Get interface metadata:
```cursed
sus stringer InterfaceInfo = get_interface_info_stringer()
vibez.spill("Interface: " + stringer.name)
vibez.spill("Methods: " + int_to_string_dynamic(len(stringer.methods)))
vibez.spill("Implementers: " + int_to_string_dynamic(len(stringer.implementers)))
```

#### implements_interface_*()
Check interface implementation:
```cursed
if implements_interface_int(42, "Stringer") {
    vibez.spill("Integer implements Stringer interface")
}
```

### Memory Layout Analysis

#### get_memory_layout_*()
Analyze memory layout:
```cursed
sus layout MemoryLayout = get_memory_layout_person()
vibez.spill("Struct size: " + int_to_string_dynamic(layout.size))
vibez.spill("Alignment: " + int_to_string_dynamic(layout.align))
vibez.spill("Padding: " + int_to_string_dynamic(layout.padding_bytes))
```

### Dynamic Object Creation

#### create_instance_by_name()
Create instances dynamically:
```cursed
sus new_int tea = create_instance_by_name("normie")
// new_int = "0"

sus new_struct tea = create_instance_by_name("PersonStruct")
// new_struct = "PersonStruct{name:\"\", age:0, active:false, score:0.0}"
```

#### create_struct_instance()
Create struct with field values:
```cursed
sus field_values []tea
field_values = append(field_values, "Bob")
field_values = append(field_values, "25")
field_values = append(field_values, "true")
field_values = append(field_values, "88.5")

sus person tea = create_struct_instance("PersonStruct", field_values)
// person = "PersonStruct{name:\"Bob\", age:25, active:true, score:88.5}"
```

### Type Registry

#### initialize_reflection_system()
Initialize the global type registry:
```cursed
initialize_reflection_system()
```

#### register_type() / register_interface()
Register custom types and interfaces:
```cursed
register_type(custom_type_info)
register_interface(custom_interface_info)
```

#### lookup_type_by_name()
Look up registered types:
```cursed
sus type_info TypeInfo = lookup_type_by_name("normie")
if type_info.name != "unknown" {
    vibez.spill("Found type: " + type_info.name)
}
```

## Usage Examples

### Basic Type Introspection
```cursed
// Get type information
sus value normie = 42
sus info TypeInfo = get_type_info_int(value)

vibez.spill("Type: " + info.name)           // "normie"
vibez.spill("Kind: " + info.kind)           // "integer"
vibez.spill("Size: " + int_to_string_dynamic(info.size))     // "4"
vibez.spill("Numeric: " + bool_to_string_dynamic(info.is_numeric)) // "true"
```

### Dynamic Method Invocation
```cursed
// Call methods dynamically
sus number normie = 42
sus str_result tea = call_method_int(number, "to_string", []tea{})
vibez.spill("String value: " + str_result)  // "String value: 42"

sus float_result tea = call_method_int(number, "to_float", []tea{})
vibez.spill("Float value: " + float_result) // "Float value: 42.0"
```

### Struct Field Manipulation
```cursed
// Create and inspect struct
sus person PersonStruct
person.name = "Alice"
person.age = 30
person.active = based
person.score = 95.5

// Get field metadata
sus fields []FieldInfo = get_struct_fields_person()
for field in fields {
    sus value tea = get_field_value_person(person, field.name)
    vibez.spill(field.name + ": " + value)
}

// Modify field dynamically
set_field_value_person(&person, "age", "31")
```

### Interface Implementation Checking
```cursed
// Check if type implements interface
sus number normie = 42
if implements_interface_int(number, "Stringer") {
    vibez.spill("Integer can be converted to string")
}

if implements_interface_int(number, "Numeric") {
    vibez.spill("Integer supports numeric operations")
}
```

### Memory Layout Analysis
```cursed
// Analyze struct memory layout
sus layout MemoryLayout = get_memory_layout_person()
vibez.spill("Total size: " + int_to_string_dynamic(layout.size) + " bytes")
vibez.spill("Alignment: " + int_to_string_dynamic(layout.align) + " bytes")
vibez.spill("Padding: " + int_to_string_dynamic(layout.padding_bytes) + " bytes")

// Field-by-field analysis
sus fields []FieldInfo = get_struct_fields_person()
for i, field in fields {
    sus offset normie = layout.field_offsets[i]
    vibez.spill(field.name + " at offset " + int_to_string_dynamic(offset))
}
```

### Dynamic Object Creation
```cursed
// Create objects by type name
sus new_int tea = create_instance_by_name("normie")
sus new_bool tea = create_instance_by_name("lit")
sus new_float tea = create_instance_by_name("meal")
sus new_string tea = create_instance_by_name("tea")

// Create struct with specific values
sus field_values []tea
field_values = append(field_values, "Bob")
field_values = append(field_values, "25")
field_values = append(field_values, "true")
field_values = append(field_values, "88.5")

sus new_person tea = create_struct_instance("PersonStruct", field_values)
vibez.spill("Created: " + new_person)
```

## Advanced Features

### Generic Type Introspection
```cursed
// Inspect generic type parameters
sus params []TypeParam = get_type_params_generic_container()
for param in params {
    vibez.spill("Type parameter: " + param.name)
    vibez.spill("Position: " + int_to_string_dynamic(param.position))
    for constraint in param.constraints {
        vibez.spill("Constraint: " + constraint)
    }
}

// Generate generic instance names
sus instance_name tea = get_generic_instance_name("GenericContainer", []tea{"normie"})
// instance_name = "GenericContainer[normie]"
```

### Type Registry Management
```cursed
// Initialize and use type registry
initialize_reflection_system()

// Look up types
sus int_type TypeInfo = lookup_type_by_name("normie")
sus bool_type TypeInfo = lookup_type_by_name("lit")

// Get all registered types
sus all_types []tea = get_all_registered_types()
vibez.spill("Registered types: " + int_to_string_dynamic(len(all_types)))
```

## Performance Considerations

- **Dynamic Method Calls**: Use sparingly in performance-critical code
- **Type Registry**: Initialize once at application startup
- **Field Access**: Direct struct access is faster than dynamic field access
- **Memory Layout**: Cache layout information for repeated operations
- **String Conversions**: Dynamic conversions allocate temporary strings

## Best Practices

1. **Initialize Early**: Call `initialize_reflection_system()` at startup
2. **Cache Type Info**: Store `TypeInfo` objects to avoid repeated lookups
3. **Validate Methods**: Check method existence before dynamic calls
4. **Handle Errors**: Always check for "method_not_found" and "field_not_found"
5. **Use Type Registry**: Register custom types for full reflection support

## Testing

Run the comprehensive test suite:
```bash
cargo run --bin cursed stdlib/reflection/test_reflection.csd
```

The test suite covers:
- ✅ Basic type information extraction (4 types)
- ✅ Dynamic method calls (20+ method tests)
- ✅ Struct field inspection (field metadata + value access)
- ✅ Interface method discovery (Stringer, Numeric, Comparable)
- ✅ Generic type parameter inspection
- ✅ Memory layout introspection (padding calculation)
- ✅ Dynamic object creation (instance creation + cloning)
- ✅ Type registry and lookup functionality
- ✅ Advanced dynamic conversions (comprehensive coverage)
- ✅ String processing and parsing utilities
- ✅ Method information and metadata validation

## Integration with CURSED Language

The reflection system integrates seamlessly with:
- **Type System**: Full support for all CURSED types
- **Struct System**: Complete struct introspection
- **Interface System**: Dynamic interface checking
- **Generic System**: Type parameter inspection
- **Memory Management**: Layout analysis and optimization
- **Testing Framework**: Comprehensive test coverage

## Future Enhancements

- [ ] Function pointer reflection
- [ ] Enum value introspection
- [ ] Array and slice element access
- [ ] Channel type reflection
- [ ] Goroutine stack introspection
- [ ] Custom attribute/tag system
- [ ] Reflection-based serialization
- [ ] Performance optimization caching

## Examples

See `stdlib/reflection/test_reflection.csd` for comprehensive usage examples and the complete test suite demonstrating all reflection capabilities.

---

This reflection system provides enterprise-grade introspection capabilities for the CURSED programming language, enabling advanced metaprogramming patterns while maintaining type safety and performance.
