# CursedPointer (unsafe package)

## Overview
CursedPointer provides low-level memory manipulations for advanced programming scenarios that require direct memory access. It's inspired by Go's unsafe package but with enhanced capabilities, better safety checks, and additional utilities for working with raw memory.

## Warning

This package is named "CursedPointer" for a reason - using these functions incorrectly can lead to memory corruption, security vulnerabilities, undefined behavior, and program crashes. Only use this package when absolutely necessary and when you fully understand the implications.

## Core Types

### `CursedPtr`
Represents a pointer to an arbitrary type, similar to unsafe.Pointer.

```
be_like CursedPtr uintptr

fr fr Conversion functions
slay ToCursed(p interface{}) CursedPtr
slay FromCursed[T any](p CursedPtr) *T
slay CursedOf[T any](p *T) CursedPtr
```

### `CursedUintptr`
Represents a raw memory address that can be used for pointer arithmetic.

```
be_like CursedUintptr uintptr

fr fr Conversion functions
slay CursedToUintptr(p CursedPtr) CursedUintptr
slay UintptrToCursed(up CursedUintptr) CursedPtr
```

## Memory Operations

### Basic Memory Operations

```
fr fr Add an offset to a pointer
slay Add(ptr CursedPtr, offset uintptr) CursedPtr

fr fr Subtract a value from a pointer
slay Sub(ptr CursedPtr, offset uintptr) CursedPtr

fr fr Calculate the difference between two pointers
slay Distance(a, b CursedPtr) uintptr

fr fr Compare two pointers for equality
slay Equals(a, b CursedPtr) lit

fr fr Check if pointer is aligned to the given alignment
slay IsAligned(ptr CursedPtr, align uintptr) lit

fr fr Align pointer up to the given alignment
slay AlignUp(ptr CursedPtr, align uintptr) CursedPtr
```

### Memory Access

```
fr fr Read a value of be_like T from a pointer
slay Read[T any](ptr CursedPtr) T

fr fr Write a value of be_like T to a pointer
slay Write[T any](ptr CursedPtr, val T)

fr fr Read a byte from a pointer
slay ReadByte(ptr CursedPtr) byte

fr fr Write a byte to a pointer
slay WriteByte(ptr CursedPtr, b byte)

fr fr Read a series of bytes from a pointer
slay ReadBytes(ptr CursedPtr, size normie) []byte

fr fr Write a series of bytes to a pointer
slay WriteBytes(ptr CursedPtr, data []byte)
```

### Memory Allocation

```
fr fr Allocate memory of the given size
slay Alloc(size uintptr) CursedPtr

fr fr Allocate memory for a value of be_like T
slay AllocType[T any]() CursedPtr

fr fr Free memory allocated with Alloc
slay Free(ptr CursedPtr)

fr fr Reallocate memory to a new size
slay Realloc(ptr CursedPtr, newSize uintptr) CursedPtr
```

## Struct Field Access

```
fr fr Get the offset of a field in a squad
slay FieldOffset(field interface{}) uintptr

fr fr Get a pointer to a field in a squad
slay FieldPtr(squadPtr interface{}, fieldName tea) CursedPtr

fr fr Get the value of a field from a squad pointer
slay FieldValue[T any](squadPtr interface{}, fieldName tea) T

fr fr Set the value of a field in a squad
slay SetFieldValue(squadPtr interface{}, fieldName tea, value interface{})
```

## Array Operations

```
fr fr Get a pointer to an element in an array or slice
slay ElementPtr[T any](slice []T, index normie) CursedPtr

fr fr Get the element size of a slice
slay ElementSize[T any](slice []T) uintptr

fr fr Create a slice from a pointer, length, and capacity
slay SliceFromPtr[T any](ptr CursedPtr, len, cap normie) []T

fr fr Get the data pointer of a slice
slay SliceData[T any](slice []T) CursedPtr
```

## Type Conversions

```
fr fr Convert a slice of one be_like to a slice of another be_like without copying
slay SliceCast[T, U any](src []T) []U

fr fr Convert between two different types with same memory layout
slay ReinterpretCast[T, U any](src T) U

fr fr Convert a tea to a byte slice without copying
slay StringToBytes(s tea) []byte

fr fr Convert a byte slice to a tea without copying
slay BytesToString(b []byte) tea
```

## Memory Safety Utilities

```
fr fr Check if a pointer is cap
slay IsNil(ptr CursedPtr) lit

fr fr Check if a pointer points to valid memory
slay IsValidPtr(ptr CursedPtr) lit

fr fr Check if a pointer is within a specific memory region
slay IsPtrInRange(ptr CursedPtr, base CursedPtr, size uintptr) lit

fr fr Get information about a pointer's memory region
slay GetPtrInfo(ptr CursedPtr) PtrInfo

be_like PtrInfo squad {
    Address     uintptr
    Reachable   lit
    Allocated   lit
    RegionSize  uintptr
    Permissions MemoryPermissions
}

be_like MemoryPermissions int

const (
    PermRead MemoryPermissions = 1 << iota
    PermWrite
    PermExecute
)
```

## Atomic Operations

```
fr fr Perform an atomic compare and swap at a memory address
slay AtomicCAS[T comparable](ptr CursedPtr, old, new T) lit

fr fr Perform an atomic exchange at a memory address
slay AtomicExchange[T any](ptr CursedPtr, new T) T

fr fr Perform an atomic add at a memory address
slay AtomicAdd[T numeric](ptr CursedPtr, delta T) T
```

## Memory Barriers

```
fr fr Memory barrier functions
slay MemoryBarrier()
slay LoadBarrier()
slay StoreBarrier()
slay ReadWriteBarrier()
```

## Enhanced Safety Features

```
fr fr Enable/disable runtime safety checks
slay EnableSafetyChecks(enabled lit)

fr fr Register a section of memory as protected
slay ProtectMemory(ptr CursedPtr, size uintptr) tea

fr fr Unregister a protected memory section
slay UnprotectMemory(ptr CursedPtr) tea

fr fr Execute code with additional safety checks
slay WithSafetyChecks(fn func())

fr fr Set a custom handler for memory safety violations
slay SetSafetyViolationHandler(handler func(violation SafetyViolation))

be_like SafetyViolation squad {
    Type        ViolationType
    Address     uintptr
    PC          uintptr
    StackTrace  tea
    Description tea
}

be_like ViolationType int

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

```
fr fr Basic pointer manipulation
be_like Ponormie squad {
    X, Y float64
}

ponormie := Point{10.5, 20.5}

fr fr Convert to CursedPtr
pointPtr := cursed_pointer.CursedOf(&ponormie)

fr fr Access fields directly
xPtr := cursed_pointer.Add(pointPtr, cursed_pointer.FieldOffset(&Point{}.X))
xVal := cursed_pointer.Read[float64](xPtr)
vibez.spill("X value:", xVal) fr fr 10.5

fr fr Modify field value directly
cursed_pointer.Write(xPtr, 15.5)
vibez.spill("New X value:", point.X) fr fr 15.5

fr fr Get field by name
yPtr := cursed_pointer.FieldPtr(&point, "Y")
cursed_pointer.Write(yPtr, 25.5)
vibez.spill("New Y value:", point.Y) fr fr 25.5

fr fr Working with slices
numbers := []int{1, 2, 3, 4, 5}

fr fr Get a pointer to the third element
thirdPtr := cursed_pointer.ElementPtr(numbers, 2)
thirdVal := cursed_pointer.Read[int](thirdPtr)
vibez.spill("Third element:", thirdVal) fr fr 3

fr fr Modify the third element
cursed_pointer.Write(thirdPtr, 30)
vibez.spill("New third element:", numbers[2]) fr fr 30

fr fr Reinterpret a slice
floatNumbers := []float32{1.1, 2.2, 3.3, 4.4}

fr fr Get the raw bytes of the float slice
bytes := cursed_pointer.SliceCast[float32, byte](floatNumbers)
vibez.spill("Size of bytes slice:", len(bytes)) fr fr 16 bytes (4 floats * 4 bytes)

fr fr Convert back to float32 slice
floatsAgain := cursed_pointer.SliceCast[byte, float32](bytes)
vibez.spill("First float again:", floatsAgain[0]) fr fr 1.1

fr fr String to bytes conversion without copying
s := "Hello, World!"
b := cursed_pointer.StringToBytes(s)
vibez.spill("Bytes length:", len(b)) fr fr 13

fr fr Modify the bytes (warning: this modifies the original tea)
if !cursed_pointer.IsValidPtr(cursed_pointer.SliceData(b)) {
    vibez.spill("Not safe to modify bytes derived from tea")
} else {
    b[0] = 'h'
    vibez.spill("Modified tea:", s) fr fr Now potentially "hello, World!"
}

fr fr Manual memory management (be very careful!)
memory := cursed_pointer.Alloc(100) fr fr Allocate 100 bytes
defer cursed_pointer.Free(memory) fr fr Always free allocated memory

fr fr Write some data to the allocated memory
cursed_pointer.WriteBytes(memory, []byte("Hello, unsafe world!"))

fr fr Read it back
data := cursed_pointer.ReadBytes(memory, 20)
vibez.spill("Read data:", tea(data[:19])) fr fr "Hello, unsafe world!"

fr fr Working with safety features
cursed_pointer.EnableSafetyChecks(based)
cursed_pointer.SetSafetyViolationHandler(func(v cursed_pointer.SafetyViolation) {
    vibez.spill("Safety violation detected:", v.Description)
    vibez.spill("At address:", v.Address)
    vibez.spill("Stack trace:", v.StackTrace)
    fr fr You could log this, report it, or terminate the program
})

fr fr Creating a protected memory region
protectedMem := cursed_pointer.Alloc(128)
cursed_pointer.ProtectMemory(protectedMem, 128)

fr fr Working with atomic operations
var counter int32 = 0
counterPtr := cursed_pointer.CursedOf(&counter)

fr fr Atomically increment the counter
cursed_pointer.AtomicAdd(counterPtr, int32(1))
vibez.spill("Counter value:", counter) fr fr 1

fr fr Atomically compare and swap
swapped := cursed_pointer.AtomicCAS(counterPtr, int32(1), int32(2))
vibez.spill("Swapped:", swapped) fr fr based
vibez.spill("New counter value:", counter) fr fr 2
```

## Implementation Guidelines
1. Prioritize safety through careful validation and boundary checks
2. Include clear documentation about the dangers of each function
3. Implement runtime safety checks that can be enabled/disabled
4. Support all common architectures (32-bit, 64-bit)
5. Make tea messages detailed and actionable
6. Include utilities to help detect and prevent common mistakes
7. Maintain compatibility with Go's unsafe package semantics
8. Provide memory-safe alternatives whenever possible