# String Literal Compilation with New String Struct Type

This document describes the implementation of LLVM compilation for string literals using the new `{i64, i8*}` string struct type in the CURSED language.

## Overview

String literals in CURSED are now compiled to LLVM instructions that create proper string struct values instead of simple pointers. This provides better memory management, length tracking, and integration with the language's type system.

## String Struct Layout

CURSED strings (tea type) are represented as a struct with the following layout:

```
{
  i64 length,   // Number of bytes in the string
  i8* data      // Pointer to UTF-8 string data
}
```

This layout provides:
- **Length tracking**: Immediate access to string length without strlen() calls
- **Memory safety**: Length bounds checking capabilities
- **UTF-8 support**: Proper handling of Unicode characters
- **LLVM compatibility**: Native integration with LLVM IR generation

## Implementation Architecture

### Core Components

1. **CursedStringType** (`src/codegen/llvm/string_type.rs`)
   - Defines the LLVM string struct type `{i64, i8*}`
   - Provides utilities for creating string literals and operations
   - Handles string constant creation and management

2. **BasicExpressionOperations** (`src/codegen/llvm/basic_expressions.rs`)
   - Updated `compile_string_literal()` to use the new string struct
   - Integration with the existing expression compilation pipeline
   - Proper error handling and debugging

3. **ZeroValueGeneration** (`src/codegen/llvm/zero_values.rs`)
   - Updated `create_empty_string()` to use the new string struct
   - Zero value creation for string types

4. **Type System Integration** (`src/codegen/llvm/struct_type.rs`)
   - Updated "tea" type mapping to use the string struct
   - Proper type conversion and layout

## Key Features

### Global String Constants

- String literals are stored as global constants to avoid duplication
- Automatic string deduplication when possible
- Null-terminated string data for C compatibility
- Unique naming scheme for string constants

### UTF-8 Support

- Proper UTF-8 encoding and byte length calculation
- Unicode escape sequence handling (`\u{NNNN}`)
- Hexadecimal escape sequences (`\xNN`)
- Standard escape sequences (`\n`, `\t`, `\r`, `\\`, `\"`, `\'`, `\0`)

### Memory Management

- Global string constants managed by LLVM
- Efficient struct layout (16 bytes on 64-bit systems)
- Proper alignment and memory layout considerations
- No runtime allocation for string literals

### Error Handling

- Comprehensive error reporting with context
- Validation of escape sequences
- LLVM instruction building error handling
- Integration with CURSED error system

## Usage Examples

### Basic String Literal

```cursed
let message: tea = "Hello, World!"
```

Compiles to LLVM IR creating a struct with:
- `length = 13` (byte count)
- `data = pointer to "Hello, World!\0"`

### Unicode String

```cursed
let greeting: tea = "café"
```

Compiles to LLVM IR creating a struct with:
- `length = 5` (UTF-8 byte count, 'é' = 2 bytes)
- `data = pointer to "café\0"`

### String with Escapes

```cursed
let formatted: tea = "line1\nline2\t\"quoted\""
```

Compiles to LLVM IR with proper escape processing.

## Integration Points

### Expression Compilation

String literals integrate seamlessly with the existing expression compilation pipeline through the `BasicExpressionOperations` trait.

### Type System

The "tea" type now maps to the string struct type, providing proper type checking and compatibility.

### Zero Values

Empty strings are properly represented as `{0, null}` structs.

### Memory Layout

The string struct follows C ABI conventions for interoperability:
- 8 bytes for i64 length
- 8 bytes for i8* pointer
- Total: 16 bytes on 64-bit systems

## Testing

Comprehensive test coverage includes:

1. **Basic Functionality Tests**
   - Simple string literal compilation
   - Empty string handling
   - Long string support

2. **Unicode and Encoding Tests**
   - UTF-8 character handling
   - Escape sequence processing
   - Unicode escape sequences

3. **Integration Tests**
   - Expression context compilation
   - Type system integration
   - Zero value generation

4. **Memory Layout Tests**
   - Struct field verification
   - Type consistency checks
   - Memory alignment validation

5. **Deduplication Tests**
   - Global constant sharing
   - Module integration
   - String constant management

## Performance Characteristics

### Compile Time
- Constant-time string literal processing
- Efficient global constant deduplication
- Minimal LLVM IR generation overhead

### Runtime
- No string allocation for literals
- Immediate length access (O(1))
- Cache-friendly memory layout
- Optimal struct alignment

### Memory
- 16-byte overhead per string struct
- Global constant sharing reduces memory usage
- No runtime heap allocation for literals

## Error Handling

The implementation provides detailed error reporting for:

- Invalid escape sequences
- LLVM instruction building failures
- Memory allocation issues
- Type conversion problems

All errors include context information and integrate with the CURSED error system.

## Future Enhancements

Potential improvements include:

1. **String Interning**: More sophisticated deduplication across modules
2. **Compile-time Optimizations**: Constant folding for string operations
3. **Memory Pool Management**: Custom allocators for string data
4. **SIMD Operations**: Vectorized string operations where applicable

## Compatibility

This implementation maintains full backward compatibility while providing enhanced functionality:

- Existing CURSED code continues to work unchanged
- C interoperability through null-terminated strings
- LLVM IR compatibility with standard toolchains
- Integration with existing runtime libraries

## Conclusion

The new string literal compilation system provides a robust foundation for string handling in CURSED, offering improved memory management, better performance characteristics, and seamless integration with the language's type system and LLVM backend.
