# Typez Module

## Why This Module Exists

The `typez` module provides comprehensive runtime type information, type reflection, and dynamic type operations for CURSED applications. While CURSED's compile-time type system ensures memory safety and performance, runtime type information enables powerful patterns like serialization, generic programming, plugin systems, and dynamic code generation.

The module exists because:
- **Dynamic Programming**: Applications need runtime type introspection for serialization, deserialization, and generic data processing
- **Plugin Architectures**: Dynamic loading and type-safe plugin systems require runtime type checking and interface verification
- **Development Tools**: IDEs, debuggers, and profiling tools need comprehensive type information for analysis and visualization
- **Generic Algorithms**: Complex generic algorithms benefit from runtime type information to optimize for specific types
- **Interoperability**: FFI and language bridges require detailed type layout and calling convention information

## Why Testing Is Critical

Type system testing is absolutely essential because:
- **Type Safety Guarantees**: Incorrect type information can break CURSED's memory safety guarantees and lead to crashes or security vulnerabilities
- **ABI Compatibility**: Type layout information affects binary compatibility and must match across compilation units
- **Performance Implications**: Type information affects optimization decisions and runtime performance characteristics
- **Serialization Correctness**: Incorrect type metadata can corrupt serialized data or cause deserialization failures
- **Generic Algorithm Safety**: Runtime type checks prevent generic algorithms from operating on incompatible types

## Implementation Rationale

### Key Design Decisions:

**1. Zero-Runtime-Cost When Unused**
- Type information is generated only when explicitly requested
- Compile-time elimination of unused type metadata
- Lazy loading of type information reduces startup time and memory usage

**2. Complete Type System Coverage**
- Support for all CURSED types: primitives, structs, enums, arrays, functions, generics
- Generic type instantiation tracking with parameter information
- Interface and trait object type information with vtable details

**3. Memory-Safe Reflection**
- All type operations maintain CURSED's memory safety guarantees
- Type-safe field access prevents buffer overflows and use-after-free
- Ownership tracking for reflected values prevents double-free errors

**4. Performance-Optimized Implementation**
- Type information cached and reused across operations
- Fast type comparison using type IDs rather than string comparisons
- Optimized field lookup using hash tables and binary search

## API Reference

### Core Type Information

#### `TypeInfo`
**Purpose**: Complete runtime information about any CURSED type
**Contents**: Size, alignment, layout, fields, methods, generic parameters
**Performance**: Cached and optimized for repeated access

```cursed
sus type_info = typez.type_of<MyStruct>()
vibez.spill("Type name:", type_info.name())
vibez.spill("Size:", type_info.size_bytes())
vibez.spill("Alignment:", type_info.alignment())
vibez.spill("Kind:", type_info.kind())  # "struct", "enum", "array", etc.
```

#### `type_of<T>() TypeInfo`
**Purpose**: Gets complete type information for any type T
**Compile-Time**: Type parameter resolved at compile time for zero runtime cost
**Coverage**: Works with all CURSED types including generics

#### `type_from_value(value: any) TypeInfo`
**Purpose**: Gets type information from a runtime value
**Dynamic**: Determines type information from actual value
**Use Case**: Working with heterogeneous data or plugin interfaces

### Type Inspection

#### `StructInfo`
**Purpose**: Detailed information about struct types
**Fields**: Field names, types, offsets, visibility
**Methods**: Method signatures and callable information

```cursed
sus struct_info = type_info.as_struct() fam {
    vibez.spill("Not a struct type")
    damn
}

# Inspect struct fields
bestie (field in struct_info.fields()) {
    vibez.spill("Field:", field.name, "type:", field.type.name(), 
               "offset:", field.offset_bytes)
}

# Inspect methods
bestie (method in struct_info.methods()) {
    vibez.spill("Method:", method.name, "signature:", method.signature())
}
```

#### `EnumInfo`
**Purpose**: Information about enum types and variants
**Variants**: All possible enum values with discriminant information
**Pattern Matching**: Support for runtime pattern matching

```cursed
sus enum_info = type_info.as_enum() fam {
    vibez.spill("Not an enum type")
    damn
}

bestie (variant in enum_info.variants()) {
    vibez.spill("Variant:", variant.name, "discriminant:", variant.discriminant)
    ready (variant.has_data()) {
        vibez.spill("  Data type:", variant.data_type.name())
    }
}
```

#### `ArrayInfo`
**Purpose**: Information about array types
**Elements**: Element type, length, stride information
**Dynamic Arrays**: Support for both fixed and dynamic arrays

### Value Reflection

#### `Value`
**Purpose**: Type-safe wrapper for reflected values with runtime type information
**Safety**: Maintains ownership and borrowing rules at runtime
**Operations**: Get, set, call methods, access fields

```cursed
# Create reflected value
sus my_struct = MyStruct{name: "test", value: 42}
sus reflected_value = typez.reflect(my_struct)

# Access fields by name
sus name_field = reflected_value.get_field("name") fam {
    vibez.spill("Field 'name' not found")
    damn
}
sus name_value = name_field.as_string() fam {
    vibez.spill("Field 'name' is not a string")
    damn
}
vibez.spill("Name:", name_value)
```

#### `get_field(name: tea) yikes<Value>`
**Purpose**: Access struct field by name with type safety
**Error Handling**: Returns error if field doesn't exist or is inaccessible
**Performance**: Optimized field lookup with caching

#### `set_field(name: tea, value: Value) yikes<fam>`
**Purpose**: Set struct field value with type checking
**Type Safety**: Ensures new value is compatible with field type
**Ownership**: Properly handles ownership transfer

#### `call_method(name: tea, args: []Value) yikes<Value>`
**Purpose**: Call method by name with runtime type checking
**Arguments**: Validates argument types and count
**Return Value**: Returns method result as reflected value

### Generic Type Information

#### `GenericInfo`
**Purpose**: Information about generic type instantiations
**Parameters**: Type parameters and their concrete types
**Constraints**: Generic constraints and bounds

```cursed
sus generic_info = type_info.as_generic() fam {
    vibez.spill("Not a generic type")
    damn
}

vibez.spill("Generic base:", generic_info.base_type().name())
bestie (param in generic_info.type_parameters()) {
    vibez.spill("Parameter:", param.name, "=", param.concrete_type.name())
}
```

### Type Comparison and Compatibility

#### `is_compatible_with(other: TypeInfo) bool`
**Purpose**: Checks if one type can be used in place of another
**Rules**: Follows CURSED's subtyping and interface implementation rules
**Use Case**: Plugin loading, dynamic dispatch, serialization

#### `is_assignable_from(other: TypeInfo) bool`
**Purpose**: Checks if a value of one type can be assigned to another
**Safety**: Ensures assignment maintains memory safety
**Coercion**: Considers automatic type coercions

```cursed
sus string_type = typez.type_of<tea>()
sus int_type = typez.type_of<drip>()

sus can_assign = string_type.is_assignable_from(int_type)
# false - cannot assign int to string without explicit conversion
```

## Usage Examples

### Dynamic Struct Processing
```cursed
yeet "typez"

# Generic function that works with any struct
slay print_struct_info(value: any) {
    sus type_info = typez.type_from_value(value)
    sus struct_info = type_info.as_struct() fam {
        vibez.spill("Value is not a struct")
        damn
    }
    
    vibez.spill("Struct:", type_info.name())
    vibez.spill("Size:", type_info.size_bytes(), "bytes")
    
    sus reflected = typez.reflect(value)
    bestie (field in struct_info.fields()) {
        sus field_value = reflected.get_field(field.name) fam {
            vibez.spill("  ", field.name, ": <inaccessible>")
            continue
        }
        
        vibez.spill("  ", field.name, ":", field_value.to_string())
    }
}

# Usage with different struct types
squad Person {
    name: tea
    age: drip
}

squad Product {
    title: tea
    price: drip
    in_stock: lit
}

sus person = Person{name: "Alice", age: 30}
sus product = Product{title: "Book", price: 1999, in_stock: based}

print_struct_info(person)   # Automatically handles Person struct
print_struct_info(product)  # Automatically handles Product struct
```

### Generic Serialization System
```cursed
# Type-safe serialization using reflection
slay serialize_to_json(value: any) tea {
    sus type_info = typez.type_from_value(value)
    sus reflected = typez.reflect(value)
    
    damn ready (type_info.kind()) {
        when "primitive" -> serialize_primitive(reflected)
        when "struct" -> serialize_struct(reflected, type_info.as_struct())
        when "array" -> serialize_array(reflected, type_info.as_array())
        when "enum" -> serialize_enum(reflected, type_info.as_enum())
        when _ -> yikes "Unsupported type for serialization"
    }
}

slay serialize_struct(value: Value, struct_info: StructInfo) tea {
    sus json_parts []tea = ["{"]
    
    bestie (field in struct_info.fields()) {
        sus field_value = value.get_field(field.name) fam {
            continue  # Skip inaccessible fields
        }
        
        sus field_json = serialize_to_json(field_value.unwrap())
        json_parts.append("\"" + field.name + "\":" + field_json)
    }
    
    json_parts.append("}")
    damn stringz.join(json_parts, ",")
}

# Deserialization with type safety
slay deserialize_from_json<T>(json_data: tea) yikes<T> {
    sus target_type = typez.type_of<T>()
    sus parsed_json = jsonz.parse(json_data) fam {
        when _ -> yikes "Invalid JSON data"
    }
    
    sus result = deserialize_value(parsed_json, target_type) fam {
        when _ -> yikes "JSON does not match target type"
    }
    
    damn result.as<T>()
}
```

### Plugin System with Type Safety
```cursed
# Dynamic plugin loading with type validation
collab Plugin {
    slay initialize() yikes<fam>
    slay get_name() tea
    slay process(input: any) yikes<any>
}

squad PluginManager {
    loaded_plugins: map<tea, Plugin>
}

slay (pm PluginManager) load_plugin(library_path: tea) yikes<fam> {
    # Load dynamic library
    sus library = ffiz.load_library(library_path) fam {
        when _ -> yikes "Cannot load plugin library"
    }
    
    # Get plugin factory function
    sus factory_fn = library.get_function("create_plugin") fam {
        when _ -> yikes "Plugin missing create_plugin function"
    }
    
    # Create plugin instance
    sus plugin_value = factory_fn.call([])
    sus plugin_type = typez.type_from_value(plugin_value)
    
    # Verify plugin implements Plugin interface
    sus plugin_interface = typez.type_of<Plugin>()
    ready (!plugin_type.implements(plugin_interface)) {
        yikes "Plugin does not implement Plugin interface"
    }
    
    # Cast to plugin interface
    sus plugin = plugin_value.as<Plugin>() fam {
        when _ -> yikes "Plugin cast failed"
    }
    
    # Initialize and register
    plugin.initialize() fam {
        when _ -> yikes "Plugin initialization failed"
    }
    
    pm.loaded_plugins.set(plugin.get_name(), plugin)
    damn fam
}
```

### Runtime Type Generation
```cursed
# Generate types dynamically for data processing
squad DynamicStructBuilder {
    fields: []FieldDefinition
}

squad FieldDefinition {
    name: tea
    type_info: TypeInfo
    default_value: ?any
}

slay (builder DynamicStructBuilder) add_field(name: tea, field_type: TypeInfo) {
    builder.fields.append(FieldDefinition{
        name: name,
        type_info: field_type,
        default_value: fam
    })
}

slay (builder DynamicStructBuilder) build() TypeInfo {
    # Create runtime struct type
    sus struct_def = typez.create_struct_type(builder.fields)
    
    # Register type for future use
    typez.register_type(struct_def)
    
    damn struct_def
}

# Usage: create struct type from configuration
sus builder = DynamicStructBuilder{}
builder.add_field("id", typez.type_of<drip>())
builder.add_field("name", typez.type_of<tea>())
builder.add_field("active", typez.type_of<lit>())

sus dynamic_type = builder.build()

# Create instances of dynamic type
sus instance = typez.create_instance(dynamic_type)
instance.set_field("id", typez.reflect(123))
instance.set_field("name", typez.reflect("Dynamic Object"))
instance.set_field("active", typez.reflect(based))
```

### Advanced Generic Type Inspection
```cursed
# Inspect generic type parameters at runtime
slay analyze_generic_type(type_info: TypeInfo) {
    sus generic_info = type_info.as_generic() fam {
        vibez.spill("Type is not generic")
        damn
    }
    
    vibez.spill("Generic type:", generic_info.base_type().name())
    vibez.spill("Type parameters:")
    
    bestie (param in generic_info.type_parameters()) {
        vibez.spill("  ", param.name, ":", param.concrete_type.name())
        
        # Analyze constraints
        bestie (constraint in param.constraints()) {
            vibez.spill("    Constraint:", constraint.description())
        }
    }
    
    # Check for specialized implementations
    ready (generic_info.has_specialization()) {
        vibez.spill("Has specialized implementation for these parameters")
    }
}

# Usage with generic containers
sus vector_type = typez.type_of<[]drip>()
analyze_generic_type(vector_type)

sus map_type = typez.type_of<map<tea, drip>>()
analyze_generic_type(map_type)
```

## Performance Considerations

### Type Information Caching

**Efficient Type Lookup**: Use type IDs for fast comparison
```cursed
sus type_id = type_info.id()  # Fast integer comparison
ready (type_id == STRING_TYPE_ID) {
    # Handle string type
}
```

**Field Access Optimization**: Cache field lookups for repeated access
```cursed
# Slow - repeated field lookup by name
bestie (item in items) {
    sus name = item.get_field("name")  # String lookup each time
}

# Fast - cache field accessor
sus name_field = struct_info.get_field_accessor("name")
bestie (item in items) {
    sus name = name_field.get_from(item)  # Direct offset access
}
```

### Memory Management

**Reflected Value Lifecycle**: Manage reflected value lifetimes carefully
```cursed
# Dangerous - reflected value outlives original
sus get_reflected_field(obj: any) Value {
    sus reflected = typez.reflect(obj)
    damn reflected.get_field("name")  # obj may be freed!
}

# Safe - explicit lifetime management
sus get_field_safely(obj: any) tea {
    sus reflected = typez.reflect(obj)
    sus field_value = reflected.get_field("name")
    damn field_value.as_string()  # Extract value before return
}
```

### Optimization Strategies

1. **Type Information Precomputation**: Generate commonly used type information at compile time
2. **Specialized Fast Paths**: Provide optimized implementations for common type operations
3. **Lazy Loading**: Load type information only when needed
4. **Memory Pooling**: Use object pools for frequently created reflected values
5. **Inline Caching**: Cache method dispatch and field access for hot paths

## Security Considerations

### Type Safety Enforcement

**Prevent Type Confusion**: Ensure reflected operations maintain type safety
```cursed
# Dangerous - could bypass type safety
slay unsafe_cast(value: Value, target_type: TypeInfo) any {
    damn value.force_cast(target_type)  # NEVER DO THIS
}

# Safe - verify type compatibility
slay safe_cast<T>(value: Value) yikes<T> {
    sus target_type = typez.type_of<T>()
    ready (!value.type().is_compatible_with(target_type)) {
        yikes "Type cast not safe"
    }
    
    damn value.as<T>() fam {
        when _ -> yikes "Cast failed despite compatibility check"
    }
}
```

### Memory Safety in Reflection

**Ownership Tracking**: Ensure reflected values respect ownership rules
```cursed
# Memory-safe field access
slay get_field_safely(obj: any, field_name: tea) yikes<Value> {
    sus reflected = typez.reflect(obj)
    sus field_value = reflected.get_field(field_name) fam {
        when "field_not_found" -> yikes "Field does not exist"
        when "access_denied" -> yikes "Field is private"
    }
    
    # Ensure field value doesn't outlive parent object
    damn field_value.clone()  # Safe copy
}
```

### Sandboxed Reflection

**Limit Reflection Capabilities**: Restrict reflection in untrusted code
```cursed
squad SecureReflectionContext {
    allowed_types: set<TypeInfo>
    allowed_operations: set<tea>
}

slay (ctx SecureReflectionContext) safe_reflect(value: any) yikes<Value> {
    sus type_info = typez.type_from_value(value)
    ready (!ctx.allowed_types.contains(type_info)) {
        yikes "Type not allowed in secure context"
    }
    
    sus reflected = typez.reflect(value)
    reflected.set_security_context(ctx)
    damn reflected
}
```

## Error Handling Patterns

### Robust Type Operations
```cursed
slay process_dynamic_data(data: any) yikes<tea> {
    sus type_info = typez.type_from_value(data)
    
    # Handle different types safely
    damn ready (type_info.kind()) {
        when "struct" -> {
            process_struct(data, type_info.as_struct() fam {
                when _ -> yikes "Invalid struct type information"
            })
        }
        when "array" -> {
            process_array(data, type_info.as_array() fam {
                when _ -> yikes "Invalid array type information"
            })
        }
        when "primitive" -> {
            process_primitive(data, type_info)
        }
        when _ -> yikes "Unsupported data type: " + type_info.name()
    }
}

slay process_struct(data: any, struct_info: StructInfo) yikes<tea> {
    sus reflected = typez.reflect(data)
    sus results []tea = []
    
    bestie (field in struct_info.fields()) {
        sus field_value = reflected.get_field(field.name) fam {
            when "field_not_found" -> {
                vibez.spill("Warning: Field", field.name, "not accessible")
                continue
            }
        }
        
        sus processed = process_dynamic_data(field_value.unwrap()) fam {
            when _ -> {
                vibez.spill("Warning: Could not process field", field.name)
                continue
            }
        }
        
        results.append(field.name + ":" + processed)
    }
    
    damn "{" + stringz.join(results, ",") + "}"
}
```

## Integration with CURSED Ecosystem

### Compiler Integration
```cursed
# Type information generated by compiler
# Available through typez API at runtime
sus compile_time_info = typez.get_compile_time_info<MyType>()
vibez.spill("Generated at compile time:", compile_time_info.generation_timestamp)
```

### Memory Management Integration
```cursed
# Reflected values integrate with arena allocators
sus arena = memoryz.arena("reflection")
sus reflected = typez.reflect_in_arena(value, arena)
# Automatically cleaned up when arena is freed
```

### Error System Integration
```cursed
# Type errors integrate with CURSED error handling
sus result = typez.safe_cast<tea>(value) fam {
    when "type_mismatch" -> handle_type_error()
    when "null_value" -> handle_null_value()
    when "access_denied" -> handle_security_error()
}
```

### Concurrency Integration
```cursed
# Type operations are thread-safe
go {
    sus type_info = typez.type_of<SharedData>()
    # Safe to access type information concurrently
}
```

The typez module provides comprehensive runtime type information while maintaining CURSED's safety guarantees and performance characteristics. It enables powerful dynamic programming patterns while preserving the benefits of static typing.
