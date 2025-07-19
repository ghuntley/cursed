# Runtime Core Module

Pure CURSED implementation of the core runtime value system essential for compiler self-hosting.

## Overview

The runtime_core module provides fundamental value types, conversions, and runtime operations needed by the CURSED compiler for self-hosting. This module is FFI-free and implemented entirely in pure CURSED.

## Key Features

### Value System
- **RuntimeValue**: Union type supporting integer, float, string, boolean, and nil values
- **Type Checking**: Runtime type validation and conversion
- **Value Creation**: Parse values from string representations
- **Type Introspection**: Get type information at runtime

### Core Operations
- `runtime_value_create(data, type)` - Create runtime values from string data
- `runtime_convert_to_string(value)` - Convert any value to string representation
- `runtime_type_check(value, expected)` - Validate value types
- `runtime_get_type(value)` - Get type name for any value

### String Processing
- `string_length(input)` - Calculate string length
- `char_at(input, index)` - Get character at position
- `integer_to_string(value)` - Convert integers to strings
- `float_to_string(value)` - Convert floats to strings

### Memory Management
- `runtime_allocate_memory(size)` - Interface with GC allocation
- `runtime_deallocate_memory(pointer)` - Interface with GC deallocation
- Memory operations integrate with CURSED's garbage collection system

### Error Handling
- `runtime_create_error(message, type)` - Create error values
- `runtime_is_error(value)` - Check if value represents an error
- Supports the runtime error propagation system

## Usage Examples

```cursed
yeet "runtime_core"

# Create and convert values
sus int_val RuntimeValue = runtime_value_create("42", "integer")
sus str_rep tea = runtime_convert_to_string(int_val)  # "42"

# Type checking
lowkey runtime_type_check(int_val, "integer") {
    vibez.spill("Value is an integer")
}

# String operations
sus length normie = string_length("hello world")  # 11
sus representation tea = integer_to_string(123)   # "123"
```

## Testing

Comprehensive test suite validates all core operations:

```bash
cargo run --bin cursed stdlib/runtime_core/test_runtime_core.csd
```

## Self-Hosting Significance

This module is critical for compiler self-hosting as it provides:

1. **Value Representation**: How the compiler represents values internally
2. **Type System**: Runtime type checking and conversion
3. **String Processing**: Essential for parsing and code generation
4. **Memory Interface**: Integration with garbage collection
5. **Error Handling**: Runtime error representation and propagation

The pure CURSED implementation ensures the compiler can fully bootstrap itself without external dependencies.

## Implementation Details

- **FFI-Free**: No external dependencies or foreign function interfaces
- **Type Safety**: Comprehensive type checking and validation
- **Memory Safe**: Integrates with CURSED's garbage collection system
- **Performance**: Optimized for compiler bootstrap scenarios
- **Extensible**: Designed to support additional value types as needed

## Integration

This module integrates with:
- **Compiler Core**: Provides value system for AST and type checking
- **Memory Management**: Works with GC for safe memory operations  
- **Error System**: Supports runtime error handling and propagation
- **String Processing**: Essential for lexical analysis and code generation
- **Bootstrap Process**: Core component of compiler self-hosting
