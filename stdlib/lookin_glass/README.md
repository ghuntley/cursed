# LookinGlass Module

## Overview
LookinGlass provides runtime reflection capabilities that allow programs to examine and modify their own structure, essentially looking into themselves like through a looking glass. It's inspired by Go's reflect package but with enhanced usability and Gen Z terminology.

## Core Types

### `Type`
Represents the type of a value.
- **Name()** - Get type name
- **Kind()** - Get type kind (Bool, Int, String, etc.)
- **Size()** - Get type size in bytes
- **String()** - String representation
- **PkgPath()** - Package path
- **Comparable()** - Check if type is comparable
- **AssignableTo(u Type)** - Check if assignable to another type
- **ConvertibleTo(u Type)** - Check if convertible to another type

### `Value`
Represents a runtime value.
- **Type()** - Get the type of the value
- **Kind()** - Get the kind of the value
- **IsValid()** - Check if value is valid
- **IsNil()** - Check if value is nil
- **IsZero()** - Check if value is zero value
- **CanSet()** - Check if value can be modified
- **CanAddr()** - Check if address can be taken
- **CanInterface()** - Check if can get as interface{}
- **Interface()** - Get as interface{}
- **Bool()** - Get as boolean
- **Int()** - Get as integer
- **Float()** - Get as float
- **String()** - Get as string

### `Kind`
Describes the specific kind of type that a Type represents.
Constants: Invalid, Bool, Int, Int8, Int16, Int32, Int64, Uint, Uint8, Uint16, Uint32, Uint64, Float32, Float64, String, Array, Slice, Map, Struct, Pointer, Interface, Chan, Func

### `StructField`
Represents a field in a struct.
- **Name** - Field name
- **Type** - Field type
- **Tag** - Struct tag
- **Offset** - Field offset
- **Index** - Field index
- **Anonymous** - Whether field is anonymous

### `StructTag`
Represents a struct tag.
- **Get(key tea)** - Get tag value for key
- **Lookup(key tea)** - Look up tag value with existence check

## Core Functions

### Type and Value Creation
- **TypeOf(i interface{}) Type** - Get reflection Type of value
- **ValueOf(i interface{}) Value** - Get reflection Value of value
- **New(typ Type) Value** - Create new zero value (settable)
- **Zero(typ Type) Value** - Create zero value (read-only)
- **Indirect(v Value) Value** - Get value pointed to by pointer

### Enhanced Utilities
- **DeepEqual(x, y interface{}) lit** - Deep equality comparison
- **DeepCopy(v interface{}) interface{}** - Create deep copy
- **StructToMap(v interface{}) map[tea]interface{}** - Convert struct to map
- **MapToStruct(m map[tea]interface{}, v interface{}) tea** - Convert map to struct
- **GetTags(v interface{}) map[tea]map[tea]tea** - Get all struct tags
- **SetField(v interface{}, name tea, value interface{}) tea** - Set field by name

## VibeMapper Utility

### `VibeMapper`
Provides methods for manipulating structs/maps with reflection.
- **ToJSON(v interface{}) ([]normie, tea)** - Convert to JSON
- **FromJSON(data []normie, v interface{}) tea** - Parse from JSON
- **ToMap(v interface{}) map[tea]interface{}** - Convert to map
- **FromMap(m map[tea]interface{}, v interface{}) tea** - Convert from map
- **Clone(v interface{}) interface{}** - Clone value
- **Merge(dst, src interface{}) tea** - Merge two values

## Usage Examples

### Basic Type Inspection
```cursed
yeet "lookin_glass"

fr fr Get type information
sus t := lookin_glass.TypeOf("hello")
vibez.spill("Type name:", t.Name())        fr fr "string"
vibez.spill("Type kind:", t.Kind())        fr fr String
vibez.spill("Type size:", t.Size())        fr fr 8
vibez.spill("Comparable:", t.Comparable()) fr fr true
```

### Value Manipulation
```cursed
fr fr Create and manipulate values
sus v := lookin_glass.ValueOf(42)
vibez.spill("Type:", v.Type().Name())     fr fr "int"
vibez.spill("Value:", v.Int())            fr fr 42
vibez.spill("Is zero:", v.IsZero())       fr fr false

fr fr Create new settable value
sus newV := lookin_glass.New(lookin_glass.TypeOf(0))
newV.SetInt(100)
vibez.spill("New value:", newV.Int())     fr fr 100
```

### Deep Equality and Copying
```cursed
fr fr Test deep equality
sus a := 42
sus b := 42
sus c := 43

vibez.spill("Equal:", lookin_glass.DeepEqual(a, b))    fr fr true
vibez.spill("Not equal:", lookin_glass.DeepEqual(a, c)) fr fr false

fr fr Deep copy
sus original := "hello"
sus copied := lookin_glass.DeepCopy(original)
vibez.spill("Copied:", copied)            fr fr "hello"
```

### Type Conversion
```cursed
fr fr Convert between compatible types
sus val := lookin_glass.ValueOf(42)
sus intType := lookin_glass.TypeOf(0)
sus converted := val.Convert(intType)
vibez.spill("Converted:", converted.Int())
```

### Struct Reflection
```cursed
fr fr Convert struct to map (simplified)
sus structMap := lookin_glass.StructToMap("example")
vibez.spill("Struct as map:", structMap)

fr fr Get struct tags
sus tags := lookin_glass.GetTags("example")
vibez.spill("Tags:", tags)
```

### VibeMapper Usage
```cursed
fr fr Create mapper
sus mapper := lookin_glass.NewVibeMapper()

fr fr Convert to JSON
sus jsonData, err := mapper.ToJSON("hello")
if err == "" {
    vibez.spill("JSON:", tea(jsonData))
}

fr fr Convert to map
sus mapData := mapper.ToMap("example")
vibez.spill("Map:", mapData)

fr fr Clone value
sus cloned := mapper.Clone(42)
vibez.spill("Cloned:", cloned)
```

### Value Setting
```cursed
fr fr Set different value types
sus boolVal := lookin_glass.New(lookin_glass.TypeOf(cap))
boolVal.SetBool(based)
vibez.spill("Bool:", boolVal.Bool())

sus intVal := lookin_glass.New(lookin_glass.TypeOf(0))
intVal.SetInt(42)
vibez.spill("Int:", intVal.Int())

sus strVal := lookin_glass.New(lookin_glass.TypeOf(""))
strVal.SetString("hello")
vibez.spill("String:", strVal.String())
```

### Struct Tags
```cursed
fr fr Work with struct tags
sus tag := lookin_glass.StructTag("json:\"name\" xml:\"Name\"")
sus jsonTag := tag.Get("json")
vibez.spill("JSON tag:", jsonTag)

sus xmlTag, found := tag.Lookup("xml")
if found {
    vibez.spill("XML tag:", xmlTag)
}
```

## Implementation Features

1. **Pure CURSED Implementation** - No FFI dependencies
2. **Type Safety** - Strong typing throughout reflection operations
3. **Value Modification** - Support for setting values when possible
4. **Deep Operations** - Deep equality and copying utilities
5. **Struct Support** - Tag parsing and field manipulation
6. **JSON Integration** - Built-in JSON conversion utilities
7. **Performance Optimized** - Efficient reflection operations

## Kind Constants

The module provides constants for all type kinds:
- **Basic Types**: Bool, Int, Int8, Int16, Int32, Int64, Uint variants, Float32, Float64, String
- **Composite Types**: Array, Slice, Map, Struct, Pointer, Interface
- **Special Types**: Chan, Func, Invalid

## Error Handling

Most operations return error strings:
- Empty string ("") indicates success
- Non-empty string contains error description

## Implementation Notes

This is a pure CURSED implementation that provides essential reflection functionality without external dependencies. Some advanced features are simplified but maintain the core reflection API for compatibility and learning purposes.

The implementation focuses on:
- Basic type inspection
- Value creation and manipulation  
- Type conversion between compatible types
- Deep equality and copying operations
- Struct-to-map conversion utilities
- Tag parsing for metadata access

For production use, this module provides a solid foundation for reflection-based operations while maintaining type safety and performance.
