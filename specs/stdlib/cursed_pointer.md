# CursedPointer (unsafe package)

## Overview
CursedPointer provides low-level memory manipulations for advanced programming scenarios that require direct memory access. It's inspired by Go's unsafe package but with enhanced capabilities, better safety checks, and additional utilities for working with raw memory.

## Warning

This package is named "CursedPointer" for a reason - using these functions incorrectly can lead to memory corruption, security vulnerabilities, undefined behavior, and program crashes. Only use this package when absolutely necessary and when you fully understand the implications.

## Core Types

### `CursedPtr`
Represents a pointer to an arbitrary type, similar to unsafe.Pointer.

```go
type CursedPtr uintptr

// Conversion functions
func ToCursed(p interface{}) CursedPtr
func FromCursed[T any](p CursedPtr) *T
func CursedOf[T any](p *T) CursedPtr
```

### `CursedUintptr`
Represents a raw memory address that can be used for pointer arithmetic.

```go
type CursedUintptr uintptr

// Conversion functions
func CursedToUintptr(p CursedPtr) CursedUintptr
func UintptrToCursed(up CursedUintptr) CursedPtr
```

## Memory Operations

### Basic Memory Operations

```go
// Add an offset to a pointer
func Add(ptr CursedPtr, offset uintptr) CursedPtr

// Subtract a value from a pointer
func Sub(ptr CursedPtr, offset uintptr) CursedPtr

// Calculate the difference between two pointers
func Distance(a, b CursedPtr) uintptr

// Compare two pointers for equality
func Equals(a, b CursedPtr) bool

// Check if pointer is aligned to the given alignment
func IsAligned(ptr CursedPtr, align uintptr) bool

// Align pointer up to the given alignment
func AlignUp(ptr CursedPtr, align uintptr) CursedPtr
```

### Memory Access

```go
// Read a value of type T from a pointer
func Read[T any](ptr CursedPtr) T

// Write a value of type T to a pointer
func Write[T any](ptr CursedPtr, val T)

// Read a byte from a pointer
func ReadByte(ptr CursedPtr) byte

// Write a byte to a pointer
func WriteByte(ptr CursedPtr, b byte)

// Read a series of bytes from a pointer
func ReadBytes(ptr CursedPtr, size int) []byte

// Write a series of bytes to a pointer
func WriteBytes(ptr CursedPtr, data []byte)
```

### Memory Allocation

```go
// Allocate memory of the given size
func Alloc(size uintptr) CursedPtr

// Allocate memory for a value of type T
func AllocType[T any]() CursedPtr

// Free memory allocated with Alloc
func Free(ptr CursedPtr)

// Reallocate memory to a new size
func Realloc(ptr CursedPtr, newSize uintptr) CursedPtr
```

## Struct Field Access

```go
// Get the offset of a field in a struct
func FieldOffset(field interface{}) uintptr

// Get a pointer to a field in a struct
func FieldPtr(structPtr interface{}, fieldName string) CursedPtr

// Get the value of a field from a struct pointer
func FieldValue[T any](structPtr interface{}, fieldName string) T

// Set the value of a field in a struct
func SetFieldValue(structPtr interface{}, fieldName string, value interface{})
```

## Array Operations

```go
// Get a pointer to an element in an array or slice
func ElementPtr[T any](slice []T, index int) CursedPtr

// Get the element size of a slice
func ElementSize[T any](slice []T) uintptr

// Create a slice from a pointer, length, and capacity
func SliceFromPtr[T any](ptr CursedPtr, len, cap int) []T

// Get the data pointer of a slice
func SliceData[T any](slice []T) CursedPtr
```

## Type Conversions

```go
// Convert a slice of one type to a slice of another type without copying
func SliceCast[T, U any](src []T) []U

// Convert between two different types with same memory layout
func ReinterpretCast[T, U any](src T) U

// Convert a string to a byte slice without copying
func StringToBytes(s string) []byte

// Convert a byte slice to a string without copying
func BytesToString(b []byte) string
```

## Memory Safety Utilities

```go
// Check if a pointer is nil
func IsNil(ptr CursedPtr) bool

// Check if a pointer points to valid memory
func IsValidPtr(ptr CursedPtr) bool

// Check if a pointer is within a specific memory region
func IsPtrInRange(ptr CursedPtr, base CursedPtr, size uintptr) bool

// Get information about a pointer's memory region
func GetPtrInfo(ptr CursedPtr) PtrInfo

type PtrInfo struct {
    Address     uintptr
    Reachable   bool
    Allocated   bool
    RegionSize  uintptr
    Permissions MemoryPermissions
}

type MemoryPermissions int

const (
    PermRead MemoryPermissions = 1 << iota
    PermWrite
    PermExecute
)
```

## Atomic Operations

```go
// Perform an atomic compare and swap at a memory address
func AtomicCAS[T comparable](ptr CursedPtr, old, new T) bool

// Perform an atomic exchange at a memory address
func AtomicExchange[T any](ptr CursedPtr, new T) T

// Perform an atomic add at a memory address
func AtomicAdd[T numeric](ptr CursedPtr, delta T) T
```

## Memory Barriers

```go
// Memory barrier functions
func MemoryBarrier()
func LoadBarrier()
func StoreBarrier()
func ReadWriteBarrier()
```

## Enhanced Safety Features

```go
// Enable/disable runtime safety checks
func EnableSafetyChecks(enabled bool)

// Register a section of memory as protected
func ProtectMemory(ptr CursedPtr, size uintptr) error

// Unregister a protected memory section
func UnprotectMemory(ptr CursedPtr) error

// Execute code with additional safety checks
func WithSafetyChecks(fn func())

// Set a custom handler for memory safety violations
func SetSafetyViolationHandler(handler func(violation SafetyViolation))

type SafetyViolation struct {
    Type        ViolationType
    Address     uintptr
    PC          uintptr
    StackTrace  string
    Description string
}

type ViolationType int

const (
    NullPointerDereference ViolationType = iota
    BufferOverflow
    UseAfterFree
    DoubleFree
    InvalidAlignment
    WildPointerDereference
)
```

## Usage Example

```go
// Basic pointer manipulation
type Point struct {
    X, Y float64
}

point := Point{10.5, 20.5}

// Convert to CursedPtr
pointPtr := cursed_pointer.CursedOf(&point)

// Access fields directly
xPtr := cursed_pointer.Add(pointPtr, cursed_pointer.FieldOffset(&Point{}.X))
xVal := cursed_pointer.Read[float64](xPtr)
vibez.spill("X value:", xVal) // 10.5

// Modify field value directly
cursed_pointer.Write(xPtr, 15.5)
vibez.spill("New X value:", point.X) // 15.5

// Get field by name
yPtr := cursed_pointer.FieldPtr(&point, "Y")
cursed_pointer.Write(yPtr, 25.5)
vibez.spill("New Y value:", point.Y) // 25.5

// Working with slices
numbers := []int{1, 2, 3, 4, 5}

// Get a pointer to the third element
thirdPtr := cursed_pointer.ElementPtr(numbers, 2)
thirdVal := cursed_pointer.Read[int](thirdPtr)
vibez.spill("Third element:", thirdVal) // 3

// Modify the third element
cursed_pointer.Write(thirdPtr, 30)
vibez.spill("New third element:", numbers[2]) // 30

// Reinterpret a slice
floatNumbers := []float32{1.1, 2.2, 3.3, 4.4}

// Get the raw bytes of the float slice
bytes := cursed_pointer.SliceCast[float32, byte](floatNumbers)
vibez.spill("Size of bytes slice:", len(bytes)) // 16 bytes (4 floats * 4 bytes)

// Convert back to float32 slice
floatsAgain := cursed_pointer.SliceCast[byte, float32](bytes)
vibez.spill("First float again:", floatsAgain[0]) // 1.1

// String to bytes conversion without copying
s := "Hello, World!"
b := cursed_pointer.StringToBytes(s)
vibez.spill("Bytes length:", len(b)) // 13

// Modify the bytes (warning: this modifies the original string)
if !cursed_pointer.IsValidPtr(cursed_pointer.SliceData(b)) {
    vibez.spill("Not safe to modify bytes derived from string")
} else {
    b[0] = 'h'
    vibez.spill("Modified string:", s) // Now potentially "hello, World!"
}

// Manual memory management (be very careful!)
memory := cursed_pointer.Alloc(100) // Allocate 100 bytes
defer cursed_pointer.Free(memory) // Always free allocated memory

// Write some data to the allocated memory
cursed_pointer.WriteBytes(memory, []byte("Hello, unsafe world!"))

// Read it back
data := cursed_pointer.ReadBytes(memory, 20)
vibez.spill("Read data:", string(data[:19])) // "Hello, unsafe world!"

// Working with safety features
cursed_pointer.EnableSafetyChecks(true)
cursed_pointer.SetSafetyViolationHandler(func(v cursed_pointer.SafetyViolation) {
    vibez.spill("Safety violation detected:", v.Description)
    vibez.spill("At address:", v.Address)
    vibez.spill("Stack trace:", v.StackTrace)
    // You could log this, report it, or terminate the program
})

// Creating a protected memory region
protectedMem := cursed_pointer.Alloc(128)
cursed_pointer.ProtectMemory(protectedMem, 128)

// Working with atomic operations
var counter int32 = 0
counterPtr := cursed_pointer.CursedOf(&counter)

// Atomically increment the counter
cursed_pointer.AtomicAdd(counterPtr, int32(1))
vibez.spill("Counter value:", counter) // 1

// Atomically compare and swap
swapped := cursed_pointer.AtomicCAS(counterPtr, int32(1), int32(2))
vibez.spill("Swapped:", swapped) // true
vibez.spill("New counter value:", counter) // 2
```

## Implementation Guidelines
1. Prioritize safety through careful validation and boundary checks
2. Include clear documentation about the dangers of each function
3. Implement runtime safety checks that can be enabled/disabled
4. Support all common architectures (32-bit, 64-bit)
5. Make error messages detailed and actionable
6. Include utilities to help detect and prevent common mistakes
7. Maintain compatibility with Go's unsafe package semantics
8. Provide memory-safe alternatives whenever possible