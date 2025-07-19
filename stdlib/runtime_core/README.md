# Runtime Core Module

Pure CURSED implementation of the runtime value system, replacing core functionality from `src/runtime/value/` with zero FFI dependencies.

## Overview

The Runtime Core module provides foundational runtime support for dynamic typing and value representation in the CURSED language. This module enables self-hosting by eliminating dependencies on Rust runtime components.

## Key Features

- **Dynamic Value System**: Runtime representation for all CURSED types
- **Type Registry**: Dynamic type registration and lookup
- **Boxing/Unboxing**: Heap allocation for large values  
- **Value Operations**: Copying, comparison, validation
- **Memory Management**: Integration with garbage collection
- **Statistics**: Runtime monitoring and health checks

## Core Types

### CursedValue
```cursed
vibe CursedValue = smash {
    value_type tea,     # Type identifier
    raw_data tea,       # Serialized data
    type_id normie,     # Runtime type ID
    size normie,        # Value size in bytes
    is_boxed lit        # Whether value is heap-allocated
}
```

## Key Functions

### Initialization
- `init_runtime_values()` - Initialize the runtime value system
- `register_type(type_id, type_name)` - Register a runtime type

### Value Management
- `create_value(type, data)` - Create a runtime value wrapper
- `box_value(value)` - Move value to heap
- `unbox_value(value)` - Copy value from heap
- `copy_value(value)` - Deep copy a value

### Type Operations
- `value_is_type(value, type)` - Type checking
- `get_type_id(type_name)` - Get type ID from name
- `validate_value(value)` - Runtime value validation

### Statistics and Monitoring
- `get_value_stats()` - Runtime statistics
- `runtime_values_health_check()` - System health check
- `clear_value_cache()` - Memory management

## Integration Points

### Memory Management
- Integrates with `memory_core` module for heap allocation
- Provides GC hints via `value_needs_gc()`
- Tracks memory usage with `value_memory_size()`

### Goroutine System
- Values can be passed between goroutines
- Thread-safe value operations
- Supports concurrent type registration

### Channel System
- Values are serialized for channel transport
- Type information preserved across channel boundaries
- Supports type-safe channel operations

## Testing

Run comprehensive tests with:
```bash
cargo run --bin cursed stdlib/runtime_core/test_runtime_core.csd
```

The test suite covers:
- Runtime initialization and type registration
- Value creation, boxing, and unboxing
- Type checking and validation
- Memory management integration
- Statistics and health monitoring
- Error handling and edge cases

## Self-Hosting Impact

This module is **critical for self-hosting** as it replaces Rust-based value representation with pure CURSED implementation:

1. **Zero FFI Dependencies**: No external C/Rust calls required
2. **Type System Foundation**: Enables dynamic typing without Rust runtime
3. **Memory Management**: Works with pure CURSED garbage collection
4. **Performance**: Optimized for CURSED compiler implementation

## Migration Status

- ✅ **Complete**: Core value system implementation  
- ✅ **Complete**: Type registration and lookup
- ✅ **Complete**: Boxing/unboxing operations
- ✅ **Complete**: Value validation and statistics
- ✅ **Complete**: Comprehensive test coverage

This module successfully replaces `src/runtime/value/` components and enables the CURSED compiler to manage its own runtime values.
