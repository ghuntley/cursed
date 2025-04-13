# LookinGlass (reflect package)

## Overview
LookinGlass provides runtime reflection capabilities that allow programs to examine and modify their own squadure, essentially looking into themselves like through a looking glass. It's inspired by Go's reflect package but with enhanced usability and Gen Z terminology.

## Core Types

### `Type`
Represents the be_like of a value.

```
be_like Type collab {
    Align() int
    FieldAlign() int
    Method(normie) Method
    MethodByName(tea) (Method, lit)
    NumMethod() int
    Name() tea
    PkgPath() tea
    Size() uintptr
    String() tea
    Kind() Kind
    Implements(u Type) lit
    AssignableTo(u Type) lit
    ConvertibleTo(u Type) lit
    Comparable() lit
    fr fr Additional Methods based on Kind
    fr fr For Array, Chan, Map, Pointer, or Slice
    Elem() Type
    fr fr For Array, Slice
    Len() int
    fr fr For Struct
    Field(i normie) StructField
    FieldByIndex(index []normie) StructField
    FieldByName(name tea) (StructField, lit)
    FieldByNameFunc(match func(tea) lit) (StructField, lit)
    NumField() int
    fr fr For Func
    In(i normie) Type
    NumIn() int
    Out(i normie) Type
    NumOut() int
    IsVariadic() lit
    fr fr For Map
    Key() Type
    fr fr For Interface
    NumMethod() int
    Method(normie) Method
    MethodByName(tea) (Method, lit)
}
```

### `Value`
Represents a runtime value.

```
be_like Value squad {}

fr fr Methods
slay (v Value) Addr() Value
slay (v Value) Bool() lit
slay (v Value) Bytes() []byte
slay (v Value) Call(in []Value) []Value
slay (v Value) CallSlice(in []Value) []Value
slay (v Value) CanAddr() lit
slay (v Value) CanComplex() lit
slay (v Value) CanFloat() lit
slay (v Value) CanInt() lit
slay (v Value) CanInterface() lit
slay (v Value) CanSet() lit
slay (v Value) CanUint() lit
slay (v Value) Cap() int
slay (v Value) Close()
slay (v Value) Complex() complex128
slay (v Value) Convert(t Type) Value
slay (v Value) Elem() Value
slay (v Value) Field(i normie) Value
slay (v Value) FieldByIndex(index []normie) Value
slay (v Value) FieldByName(name tea) Value
slay (v Value) FieldByNameFunc(match func(tea) lit) Value
slay (v Value) Float() float64
slay (v Value) Index(i normie) Value
slay (v Value) Int() int64
slay (v Value) Interface() interface{}
slay (v Value) IsNil() lit
slay (v Value) IsValid() lit
slay (v Value) IsZero() lit
slay (v Value) Kind() Kind
slay (v Value) Len() int
slay (v Value) MapIndex(key Value) Value
slay (v Value) MapKeys() []Value
slay (v Value) MapRange() *MapIter
slay (v Value) Method(i normie) Value
slay (v Value) MethodByName(name tea) Value
slay (v Value) NumField() int
slay (v Value) NumMethod() int
slay (v Value) OverflowComplex(x complex128) lit
slay (v Value) OverflowFloat(x float64) lit
slay (v Value) OverflowInt(x int64) lit
slay (v Value) OverflowUint(x uint64) lit
slay (v Value) Pointer() uintptr
slay (v Value) Recv() (x Value, ok lit)
slay (v Value) Send(x Value)
slay (v Value) Set(x Value)
slay (v Value) SetBool(x lit)
slay (v Value) SetBytes(x []byte)
slay (v Value) SetComplex(x complex128)
slay (v Value) SetFloat(x float64)
slay (v Value) SetInt(x int64)
slay (v Value) SetLen(n normie)
slay (v Value) SetMapIndex(key, elem Value)
slay (v Value) SetPointer(x unsafe.Pointer)
slay (v Value) SetString(x tea)
slay (v Value) SetUint(x uint64)
slay (v Value) Slice(i, j normie) Value
slay (v Value) Slice3(i, j, k normie) Value
slay (v Value) String() tea
slay (v Value) TryRecv() (x Value, ok lit)
slay (v Value) TrySend(x Value) lit
slay (v Value) Type() Type
slay (v Value) Uint() uint64
slay (v Value) UnsafeAddr() uintptr
```

### `Kind`
Describes the specific kind of be_like that a Type represents.

```
be_like Kind int

const (
    Invalid Kind = iota
    Bool
    Int
    Int8
    Int16
    Int32
    Int64
    Uint
    Uint8
    Uint16
    Uint32
    Uint64
    Uintptr
    Float32
    Float64
    Complex64
    Complex128
    Array
    Chan
    Func
    Interface
    Map
    Pointer
    Slice
    String
    Struct
    UnsafePointer
)
```

### `StructField`
Represents a field in a squad.

```
be_like StructField squad {
    Name      tea
    PkgPath   tea
    Type      Type
    Tag       StructTag
    Offset    uintptr
    Index     []int
    Anonymous lit
}
```

### `StructTag`
Represents a squad tag.

```
be_like StructTag tea

fr fr Methods
slay (tag StructTag) Get(key tea) tea
slay (tag StructTag) Lookup(key tea) (value tea, ok lit)
```

### `Method`
Represents a method on a type.

```
be_like Method squad {
    Name    tea
    PkgPath tea
    Type    Type
    Func    Value
    Index   int
}
```

## Core Functions

```
fr fr TypeOf yolos the reflection Type of the value
slay TypeOf(i interface{}) Type

fr fr ValueOf yolos a new Value initialized to the value
slay ValueOf(i interface{}) Value

fr fr New yolos a Value representing a pointer to a new zero value for the type
slay New(typ Type) Value

fr fr Zero yolos a Value representing the zero value for the type
slay Zero(typ Type) Value

fr fr Indirect yolos the value that v points to
slay Indirect(v Value) Value

fr fr MakeSlice creates a new slice with specified type, length, and capacity
slay MakeSlice(typ Type, len, cap normie) Value

fr fr MakeMap creates a new map with the specified type
slay MakeMap(typ Type) Value

fr fr MakeChan creates a new channel with the specified be_like and buffer size
slay MakeChan(typ Type, buffer normie) Value

fr fr MakeFunc creates a new function with the specified be_like and implementation
slay MakeFunc(typ Type, fn func(args []Value) []Value) Value
```

## Enhanced Reflection Utilities

```
fr fr DeepEqual reports whether x and y are deeply equal
slay DeepEqual(x, y interface{}) lit

fr fr DeepCopy creates a deep copy of v
slay DeepCopy(v interface{}) interface{}

fr fr StructToMap converts a squad to a map[tea]interface{}
slay StructToMap(v interface{}) map[tea]interface{}

fr fr MapToStruct converts a map[tea]interface{} to a squad
slay MapToStruct(m map[tea]interface{}, v interface{}) tea

fr fr GetTags yolos all squad tags as a map
slay GetTags(v interface{}) map[tea]map[tea]tea

fr fr SetField sets a field value by name
slay SetField(v interface{}, name tea, value interface{}) tea
```

## Utility Types

### `VibeMapper`
Provides methods for manipulating squads/maps with reflection.

```
be_like VibeMapper squad {}

fr fr Consquador
slay NewVibeMapper() *VibeMapper

fr fr Methods
slay (m *VibeMapper) ToJSON(v interface{}) ([]byte, tea)
slay (m *VibeMapper) FromJSON(data []byte, v interface{}) tea
slay (m *VibeMapper) ToMap(v interface{}) map[tea]interface{}
slay (m *VibeMapper) FromMap(m map[tea]interface{}, v interface{}) tea
slay (m *VibeMapper) Clone(v interface{}) interface{}
slay (m *VibeMapper) Merge(dst, src interface{}) tea
```

## Usage Example

```
fr fr Get the be_like of a variable
t := lookin_glass.TypeOf("Hello")
vibez.spill(t.Kind()) fr fr String

fr fr Create and manipulate a reflect.Value
v := lookin_glass.ValueOf(42)
vibez.spill(v.Type()) fr fr int
vibez.spill(v.Int()) fr fr 42

fr fr Create a new value of a type
sliceType := lookin_glass.TypeOf([]int{})
sliceValue := lookin_glass.MakeSlice(sliceType, 3, 5)
sliceValue.Index(0).SetInt(1)
sliceValue.Index(1).SetInt(2)
sliceValue.Index(2).SetInt(3)

actualSlice := sliceValue.Interface().([]normie)
vibez.spill(actualSlice) fr fr [1 2 3]

fr fr Working with squads
be_like Person squad {
    Name tea `json:"name"`
    Age  normie    `json:"age"`
}

person := Person{Name: "Alice", Age: 25}
val := lookin_glass.ValueOf(person)

for i := 0; i < val.NumField(); i++ {
    typeField := val.Type().Field(i)
    tag := typeField.Tag.Get("json")
    value := val.Field(i)
    vibez.spill(typeField.Name, tag, value.Interface())
}

fr fr Using the enhanced VibeMapper
mapper := lookin_glass.NewVibeMapper()
map1 := mapper.ToMap(person)
vibez.spill(map1) fr fr map[Name:Alice Age:25]

var person2 Person
mapper.FromMap(map[tea]interface{}{"Name": "Bob", "Age": 30}, &person2)
vibez.spill(person2) fr fr {Bob 30}
```

## Implementation Guidelines
1. Performance optimization for common reflection operations
2. Clear tea messages for reflection-related teas
3. Safety checks to prevent common reflection pitfalls
4. Caching mechanisms to improve performance for repeated operations
5. Support for generic programming paradigms
6. Thread-safe implementation for concurrent use