# Type System Core (type_core)

The `type_core` module provides runtime type information and type operations for CURSED programs.

## Purpose

This module implements runtime type introspection, type checking, conversion utilities, and generic type operations for dynamic type handling in CURSED.

## Main Functions

### Type Introspection
- `type_core.type_of(value)` - Get runtime type of value
- `type_core.type_name(type)` - Get human-readable type name
- `type_core.is_type(value, type)` - Check if value is of specific type
- `type_core.same_type(value1, value2)` - Check if values have same type
- `type_core.implements(value, interface)` - Check interface implementation

### Type Conversion
- `type_core.to_string(value)` - Convert any value to string
- `type_core.to_int(value)` - Convert to integer
- `type_core.to_float(value)` - Convert to floating point
- `type_core.to_bool(value)` - Convert to boolean
- `type_core.cast(value, target_type)` - Safe type casting

### Type Construction
- `type_core.create_instance(type, args)` - Create instance from type
- `type_core.clone(value)` - Deep clone value
- `type_core.default_value(type)` - Get default value for type
- `type_core.size_of(type)` - Get size of type in bytes

### Generic Operations
- `type_core.compare(value1, value2)` - Generic comparison
- `type_core.equals(value1, value2)` - Generic equality
- `type_core.hash(value)` - Generic hash function
- `type_core.serialize(value)` - Serialize to bytes
- `type_core.deserialize(bytes, type)` - Deserialize from bytes

## Usage Examples

### Basic Type Introspection

```cursed
yeet "type_core"

sus values []normie = [42, 3.14, "hello", based, cringe]

bestie value in values {
    sus type_info = type_core.type_of(value)
    sus type_name = type_core.type_name(type_info)
    vibez.spillf("Value: {}, Type: {}", value, type_name)
}

fr fr Check specific types
if type_core.is_type(42, type_core.INT_TYPE) {
    vibez.spill("42 is an integer")
}

if type_core.same_type(3.14, 2.71) {
    vibez.spill("Both are same type")
}
```

### Type Conversion

```cursed
yeet "type_core"

fr fr Safe conversions with error handling
sus string_number tea = "123"
sus maybe_int = type_core.to_int(string_number)

match maybe_int {
    Some(num) => vibez.spillf("Converted: {}", num),
    None => vibez.spill("Conversion failed")
}

fr fr Convert various types to string
sus values []normie = [42, 3.14, based, [1, 2, 3]]
bestie value in values {
    sus str_repr = type_core.to_string(value)
    vibez.spillf("{} -> '{}'", type_core.type_name(type_core.type_of(value)), str_repr)
}

fr fr Safe casting
sus float_val meal = 42.0
sus cast_result = type_core.cast(float_val, type_core.INT_TYPE)
match cast_result {
    Some(int_val) => vibez.spillf("Cast success: {}", int_val),
    None => vibez.spill("Cast failed")
}
```

### Generic Operations

```cursed
yeet "type_core"

slay generic_max<T>(a T, b T) T {
    sus comparison = type_core.compare(a, b)
    if comparison > 0 {
        damn a
    } else {
        damn b
    }
}

sus max_int = generic_max(10, 20)
sus max_float = generic_max(3.14, 2.71)
sus max_string = generic_max("apple", "banana")

vibez.spillf("Max int: {}", max_int)
vibez.spillf("Max float: {}", max_float) 
vibez.spillf("Max string: {}", max_string)

fr fr Generic equality checking
slay contains<T>(array []T, item T) lit {
    bestie element in array {
        if type_core.equals(element, item) {
            damn based
        }
    }
    damn cringe
}

sus numbers []normie = [1, 2, 3, 4, 5]
vibez.spillf("Contains 3: {}", contains(numbers, 3))
```

### Runtime Type Creation

```cursed
yeet "type_core"

fr fr Create instances dynamically
sus point_type = type_core.get_struct_type("Point")
sus point_args = {"x": 10, "y": 20}
sus point = type_core.create_instance(point_type, point_args)

vibez.spillf("Created point: {}", type_core.to_string(point))

fr fr Get default values
sus default_int = type_core.default_value(type_core.INT_TYPE)
sus default_string = type_core.default_value(type_core.STRING_TYPE)
sus default_bool = type_core.default_value(type_core.BOOL_TYPE)

vibez.spillf("Default int: {}", default_int)
vibez.spillf("Default string: '{}'", default_string)
vibez.spillf("Default bool: {}", default_bool)
```

### Interface Checking

```cursed
yeet "type_core"

collab Drawable {
    slay draw()
    slay area() meal
}

squad Circle {
    spill radius meal
}

flex Circle => Drawable {
    slay draw() { vibez.spill("Drawing circle") }
    slay area() meal { damn 3.14159 * radius * radius }
}

sus circle = Circle{radius: 5.0}
sus drawable_interface = type_core.get_interface_type("Drawable")

if type_core.implements(circle, drawable_interface) {
    vibez.spill("Circle implements Drawable")
    circle.draw()
} else {
    vibez.spill("Circle does not implement Drawable")
}
```

### Serialization and Cloning

```cursed
yeet "type_core"

squad Person {
    spill name tea
    spill age normie
    spill scores []normie
}

sus original = Person{
    name: "Alice",
    age: 30,
    scores: [85, 92, 78]
}

fr fr Deep clone
sus cloned = type_core.clone(original)
cloned.name = "Bob"
cloned.scores.push(95)

vibez.spillf("Original: {}", type_core.to_string(original))
vibez.spillf("Cloned: {}", type_core.to_string(cloned))

fr fr Serialization
sus serialized = type_core.serialize(original)
vibez.spillf("Serialized size: {} bytes", serialized.len())

sus person_type = type_core.type_of(original)
sus deserialized = type_core.deserialize(serialized, person_type)
match deserialized {
    Some(person) => vibez.spillf("Deserialized: {}", type_core.to_string(person)),
    None => vibez.spill("Deserialization failed")
}
```

## Compilation Examples

### Interpretation Mode
```bash
echo 'yeet "type_core"
sus type_name = type_core.type_name(type_core.type_of(42))
vibez.spillf("Type: {}", type_name)' > type_test.💀

./cursed-unified type_test.💀
```

### Compilation Mode
```bash
./cursed-unified --compile type_test.💀
./type_test
```

## Built-in Type Constants

### Primitive Types
- `type_core.INT_TYPE` - Integer type
- `type_core.FLOAT_TYPE` - Floating point type
- `type_core.BOOL_TYPE` - Boolean type
- `type_core.STRING_TYPE` - String type
- `type_core.CHAR_TYPE` - Character type

### Composite Types
- `type_core.ARRAY_TYPE` - Array type
- `type_core.MAP_TYPE` - Map type
- `type_core.STRUCT_TYPE` - Struct type
- `type_core.INTERFACE_TYPE` - Interface type
- `type_core.FUNCTION_TYPE` - Function type

### Special Types
- `type_core.ANY_TYPE` - Any/unknown type
- `type_core.VOID_TYPE` - Void type
- `type_core.ERROR_TYPE` - Error type
- `type_core.OPTION_TYPE` - Option/Maybe type

## Advanced Examples

### Generic Container

```cursed
yeet "type_core"

squad Container<T> {
    spill value T
    spill type_info TypeInfo
}

slay container_new<T>(value T) Container<T> {
    damn Container<T>{
        value: value,
        type_info: type_core.type_of(value)
    }
}

slay container_get_type_name<T>(container Container<T>) tea {
    damn type_core.type_name(container.type_info)
}

slay container_equals<T>(c1 Container<T>, c2 Container<T>) lit {
    damn type_core.same_type(c1.value, c2.value) &&
         type_core.equals(c1.value, c2.value)
}

sus int_container = container_new(42)
sus string_container = container_new("hello")

vibez.spillf("Int container type: {}", container_get_type_name(int_container))
vibez.spillf("String container type: {}", container_get_type_name(string_container))
```

### Dynamic Function Dispatch

```cursed
yeet "type_core"

slay dynamic_process(value normie) tea {
    sus type_info = type_core.type_of(value)
    
    if type_core.is_type(value, type_core.INT_TYPE) {
        damn "Processing integer: " + type_core.to_string(value)
    } elif type_core.is_type(value, type_core.FLOAT_TYPE) {
        damn "Processing float: " + type_core.to_string(value)
    } elif type_core.is_type(value, type_core.STRING_TYPE) {
        damn "Processing string: " + value
    } elif type_core.is_type(value, type_core.BOOL_TYPE) {
        damn "Processing boolean: " + type_core.to_string(value)
    } else {
        damn "Unknown type: " + type_core.type_name(type_info)
    }
}

sus values []normie = [42, 3.14, "test", based]
bestie value in values {
    sus result = dynamic_process(value)
    vibez.spill(result)
}
```

## Implementation Notes

- Runtime type information (RTTI) support
- Efficient type checking and comparison
- Memory-safe type conversions
- Integration with CURSED's type system
- Support for generic programming patterns

## Dependencies

- `memory` - For type metadata management
- `serialization` - For serialize/deserialize operations
- Core runtime type system
- No external dependencies

## Performance Considerations

- Efficient type lookups with caching
- Minimal overhead for type checks
- Optimized comparison operations
- Memory-efficient type metadata
- Fast serialization for simple types

## Best Practices

1. **Cache type information** for repeated checks
2. **Use specific type checks** instead of string comparisons  
3. **Handle conversion failures** gracefully
4. **Prefer compile-time types** over runtime checks when possible
5. **Use generic functions** for type-safe operations
6. **Validate types** before unsafe operations
7. **Document expected types** in function signatures

## Safety Considerations

- All type conversions are checked and safe
- No undefined behavior from invalid casts
- Proper error handling for conversion failures
- Memory safety in type operations
- Thread-safe type information access
