# CursedPointer Module

## Overview
CursedPointer provides low-level memory manipulations for advanced programming scenarios that require direct memory access. It's inspired by Go's unsafe package but with enhanced capabilities and additional safety utilities.

## Warning
⚠️ This package is named "CursedPointer" for a reason - using these functions incorrectly can lead to memory corruption, security vulnerabilities, undefined behavior, and program crashes. Only use this package when absolutely necessary and when you fully understand the implications.

## Core Types

### `CursedPtr`
Represents a pointer to an arbitrary type, similar to unsafe.Pointer.

### `CursedUintptr`
Represents a raw memory address that can be used for pointer arithmetic.

## Key Functions

### Conversion Functions
- `ToCursed(p normie) CursedPtr` - Convert integer to CursedPtr
- `FromCursed(p CursedPtr) normie` - Convert CursedPtr to integer
- `CursedOf(p normie) CursedPtr` - Get CursedPtr from value
- `CursedToUintptr(p CursedPtr) CursedUintptr` - Convert to uintptr
- `UintptrToCursed(up CursedUintptr) CursedPtr` - Convert from uintptr

### Memory Operations
- `Add(ptr CursedPtr, offset normie) CursedPtr` - Add offset to pointer
- `Sub(ptr CursedPtr, offset normie) CursedPtr` - Subtract offset from pointer
- `Distance(a, b CursedPtr) normie` - Calculate distance between pointers
- `Equals(a, b CursedPtr) lit` - Compare pointers for equality
- `IsAligned(ptr CursedPtr, align normie) lit` - Check alignment
- `AlignUp(ptr CursedPtr, align normie) CursedPtr` - Align pointer up

### Memory Access
- `ReadByte(ptr CursedPtr) normie` - Read byte from pointer
- `WriteByte(ptr CursedPtr, b normie)` - Write byte to pointer
- `ReadBytes(ptr CursedPtr, size normie) []normie` - Read multiple bytes
- `WriteBytes(ptr CursedPtr, data []normie)` - Write multiple bytes

### Safety Utilities
- `IsNil(ptr CursedPtr) lit` - Check if pointer is nil
- `IsValidPtr(ptr CursedPtr) lit` - Check if pointer is valid
- `IsPtrInRange(ptr, base CursedPtr, size normie) lit` - Check range
- `EnableSafetyChecks(enabled lit)` - Enable/disable safety checks
- `WithSafetyChecks(action slay())` - Execute with safety checks

### Atomic Operations
- `AtomicCAS(ptr CursedPtr, old, new normie) lit` - Compare and swap
- `AtomicExchange(ptr CursedPtr, new normie) normie` - Atomic exchange
- `AtomicAdd(ptr CursedPtr, delta normie) normie` - Atomic add

### String Conversion
- `StringToBytes(s tea) []normie` - Convert string to byte array
- `BytesToString(b []normie) tea` - Convert byte array to string

## Usage Example

```cursed
yeet "cursed_pointer"

fr fr Basic pointer manipulation
sus ptr := cursed_pointer.ToCursed(42)
sus newPtr := cursed_pointer.Add(ptr, 10)
sus distance := cursed_pointer.Distance(newPtr, ptr)
vibez.spill("Distance:", distance)  fr fr 10

fr fr Memory operations
sus basePtr := cursed_pointer.ToCursed(1000)
cursed_pointer.WriteByte(basePtr, 42)
sus value := cursed_pointer.ReadByte(basePtr)
vibez.spill("Read value:", value)  fr fr 42

fr fr String conversion
sus text := "hello"
sus bytes := cursed_pointer.StringToBytes(text)
sus backToText := cursed_pointer.BytesToString(bytes)
vibez.spill("Converted:", backToText)  fr fr "hello"
```

## Safety Considerations

1. Always validate pointers before use
2. Be careful with pointer arithmetic
3. Use safety checks in development
4. Test thoroughly before production use
5. Consider memory alignment requirements
6. Avoid dereferencing invalid pointers

## Implementation Notes

This is a pure CURSED implementation without FFI dependencies. Some operations are simplified but maintain the essential API for compatibility and learning purposes.
