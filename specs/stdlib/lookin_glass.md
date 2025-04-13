# LookinGlass (reflect package)

## Overview
LookinGlass provides runtime reflection capabilities that allow programs to examine and modify their own structure, essentially looking into themselves like through a looking glass. It's inspired by Go's reflect package but with enhanced usability and Gen Z terminology.

## Core Types

### `Type`
Represents the type of a value.

```go
type Type interface {
    Align() int
    FieldAlign() int
    Method(int) Method
    MethodByName(string) (Method, bool)
    NumMethod() int
    Name() string
    PkgPath() string
    Size() uintptr
    String() string
    Kind() Kind
    Implements(u Type) bool
    AssignableTo(u Type) bool
    ConvertibleTo(u Type) bool
    Comparable() bool
    // Additional Methods based on Kind
    // For Array, Chan, Map, Pointer, or Slice
    Elem() Type
    // For Array, Slice
    Len() int
    // For Struct
    Field(i int) StructField
    FieldByIndex(index []int) StructField
    FieldByName(name string) (StructField, bool)
    FieldByNameFunc(match func(string) bool) (StructField, bool)
    NumField() int
    // For Func
    In(i int) Type
    NumIn() int
    Out(i int) Type
    NumOut() int
    IsVariadic() bool
    // For Map
    Key() Type
    // For Interface
    NumMethod() int
    Method(int) Method
    MethodByName(string) (Method, bool)
}
```

### `Value`
Represents a runtime value.

```go
type Value struct {}

// Methods
func (v Value) Addr() Value
func (v Value) Bool() bool
func (v Value) Bytes() []byte
func (v Value) Call(in []Value) []Value
func (v Value) CallSlice(in []Value) []Value
func (v Value) CanAddr() bool
func (v Value) CanComplex() bool
func (v Value) CanFloat() bool
func (v Value) CanInt() bool
func (v Value) CanInterface() bool
func (v Value) CanSet() bool
func (v Value) CanUint() bool
func (v Value) Cap() int
func (v Value) Close()
func (v Value) Complex() complex128
func (v Value) Convert(t Type) Value
func (v Value) Elem() Value
func (v Value) Field(i int) Value
func (v Value) FieldByIndex(index []int) Value
func (v Value) FieldByName(name string) Value
func (v Value) FieldByNameFunc(match func(string) bool) Value
func (v Value) Float() float64
func (v Value) Index(i int) Value
func (v Value) Int() int64
func (v Value) Interface() interface{}
func (v Value) IsNil() bool
func (v Value) IsValid() bool
func (v Value) IsZero() bool
func (v Value) Kind() Kind
func (v Value) Len() int
func (v Value) MapIndex(key Value) Value
func (v Value) MapKeys() []Value
func (v Value) MapRange() *MapIter
func (v Value) Method(i int) Value
func (v Value) MethodByName(name string) Value
func (v Value) NumField() int
func (v Value) NumMethod() int
func (v Value) OverflowComplex(x complex128) bool
func (v Value) OverflowFloat(x float64) bool
func (v Value) OverflowInt(x int64) bool
func (v Value) OverflowUint(x uint64) bool
func (v Value) Pointer() uintptr
func (v Value) Recv() (x Value, ok bool)
func (v Value) Send(x Value)
func (v Value) Set(x Value)
func (v Value) SetBool(x bool)
func (v Value) SetBytes(x []byte)
func (v Value) SetComplex(x complex128)
func (v Value) SetFloat(x float64)
func (v Value) SetInt(x int64)
func (v Value) SetLen(n int)
func (v Value) SetMapIndex(key, elem Value)
func (v Value) SetPointer(x unsafe.Pointer)
func (v Value) SetString(x string)
func (v Value) SetUint(x uint64)
func (v Value) Slice(i, j int) Value
func (v Value) Slice3(i, j, k int) Value
func (v Value) String() string
func (v Value) TryRecv() (x Value, ok bool)
func (v Value) TrySend(x Value) bool
func (v Value) Type() Type
func (v Value) Uint() uint64
func (v Value) UnsafeAddr() uintptr
```

### `Kind`
Describes the specific kind of type that a Type represents.

```go
type Kind int

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
Represents a field in a struct.

```go
type StructField struct {
    Name      string
    PkgPath   string
    Type      Type
    Tag       StructTag
    Offset    uintptr
    Index     []int
    Anonymous bool
}
```

### `StructTag`
Represents a struct tag.

```go
type StructTag string

// Methods
func (tag StructTag) Get(key string) string
func (tag StructTag) Lookup(key string) (value string, ok bool)
```

### `Method`
Represents a method on a type.

```go
type Method struct {
    Name    string
    PkgPath string
    Type    Type
    Func    Value
    Index   int
}
```

## Core Functions

```go
// TypeOf returns the reflection Type of the value
func TypeOf(i interface{}) Type

// ValueOf returns a new Value initialized to the value
func ValueOf(i interface{}) Value

// New returns a Value representing a pointer to a new zero value for the type
func New(typ Type) Value

// Zero returns a Value representing the zero value for the type
func Zero(typ Type) Value

// Indirect returns the value that v points to
func Indirect(v Value) Value

// MakeSlice creates a new slice with specified type, length, and capacity
func MakeSlice(typ Type, len, cap int) Value

// MakeMap creates a new map with the specified type
func MakeMap(typ Type) Value

// MakeChan creates a new channel with the specified type and buffer size
func MakeChan(typ Type, buffer int) Value

// MakeFunc creates a new function with the specified type and implementation
func MakeFunc(typ Type, fn func(args []Value) []Value) Value
```

## Enhanced Reflection Utilities

```go
// DeepEqual reports whether x and y are deeply equal
func DeepEqual(x, y interface{}) bool

// DeepCopy creates a deep copy of v
func DeepCopy(v interface{}) interface{}

// StructToMap converts a struct to a map[string]interface{}
func StructToMap(v interface{}) map[string]interface{}

// MapToStruct converts a map[string]interface{} to a struct
func MapToStruct(m map[string]interface{}, v interface{}) error

// GetTags returns all struct tags as a map
func GetTags(v interface{}) map[string]map[string]string

// SetField sets a field value by name
func SetField(v interface{}, name string, value interface{}) error
```

## Utility Types

### `VibeMapper`
Provides methods for manipulating structs/maps with reflection.

```go
type VibeMapper struct {}

// Constructor
func NewVibeMapper() *VibeMapper

// Methods
func (m *VibeMapper) ToJSON(v interface{}) ([]byte, error)
func (m *VibeMapper) FromJSON(data []byte, v interface{}) error
func (m *VibeMapper) ToMap(v interface{}) map[string]interface{}
func (m *VibeMapper) FromMap(m map[string]interface{}, v interface{}) error
func (m *VibeMapper) Clone(v interface{}) interface{}
func (m *VibeMapper) Merge(dst, src interface{}) error
```

## Usage Example

```go
// Get the type of a variable
t := lookin_glass.TypeOf("Hello")
vibez.spill(t.Kind()) // String

// Create and manipulate a reflect.Value
v := lookin_glass.ValueOf(42)
vibez.spill(v.Type()) // int
vibez.spill(v.Int()) // 42

// Create a new value of a type
sliceType := lookin_glass.TypeOf([]int{})
sliceValue := lookin_glass.MakeSlice(sliceType, 3, 5)
sliceValue.Index(0).SetInt(1)
sliceValue.Index(1).SetInt(2)
sliceValue.Index(2).SetInt(3)

actualSlice := sliceValue.Interface().([]int)
vibez.spill(actualSlice) // [1 2 3]

// Working with structs
type Person struct {
    Name string `json:"name"`
    Age  int    `json:"age"`
}

person := Person{Name: "Alice", Age: 25}
val := lookin_glass.ValueOf(person)

for i := 0; i < val.NumField(); i++ {
    typeField := val.Type().Field(i)
    tag := typeField.Tag.Get("json")
    value := val.Field(i)
    vibez.spill(typeField.Name, tag, value.Interface())
}

// Using the enhanced VibeMapper
mapper := lookin_glass.NewVibeMapper()
map1 := mapper.ToMap(person)
vibez.spill(map1) // map[Name:Alice Age:25]

var person2 Person
mapper.FromMap(map[string]interface{}{"Name": "Bob", "Age": 30}, &person2)
vibez.spill(person2) // {Bob 30}
```

## Implementation Guidelines
1. Performance optimization for common reflection operations
2. Clear error messages for reflection-related errors
3. Safety checks to prevent common reflection pitfalls
4. Caching mechanisms to improve performance for repeated operations
5. Support for generic programming paradigms
6. Thread-safe implementation for concurrent use