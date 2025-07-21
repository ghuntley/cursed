# Runtime Core Module

The Runtime Core module (`stdlib/runtime_core/mod.csd`) provides essential runtime operations implemented in pure CURSED language for compiler self-hosting. This module eliminates FFI dependencies and provides a foundation for the CURSED runtime system.

## Overview

This module implements critical runtime operations that were previously stubs, enabling the CURSED compiler to manage its own execution environment. All implementations are in pure CURSED language following the specifications in the `specs/` directory.

## Implemented Operations

### Array Operations

- `array_get_length(arr [RuntimeValue]) normie` - Get array length with bounds checking
- `array_get_element(arr [RuntimeValue], index normie) RuntimeValue` - Safe element access
- `array_set_element(arr [RuntimeValue], index normie, value RuntimeValue) lit` - Safe element assignment
- `runtime_array_length(arr [RuntimeValue]) normie` - Runtime interface for array length
- `runtime_array_get(arr [RuntimeValue], index normie) RuntimeValue` - Runtime interface for element access
- `runtime_array_set(arr [RuntimeValue], index normie, value RuntimeValue) lit` - Runtime interface for element assignment

#### Implementation Details
- Uses linear search through array elements until null terminator found
- Implements bounds checking to prevent out-of-bounds access
- Returns `cringe` (nil) for invalid indices
- Maximum array size limited to 10,000 elements for safety

### Map Operations

- `map_has_key(map vibes[tea]RuntimeValue, key tea) lit` - Check if key exists in map
- `map_get_value(map vibes[tea]RuntimeValue, key tea) RuntimeValue` - Get value by key
- `map_set_value(map vibes[tea]RuntimeValue, key tea, value RuntimeValue) lit` - Set key-value pair
- `runtime_map_get(map vibes[tea]RuntimeValue, key tea) RuntimeValue` - Runtime interface for map access
- `runtime_map_set(map vibes[tea]RuntimeValue, key tea, value RuntimeValue) lit` - Runtime interface for map assignment

#### Implementation Details
- Uses linear search through map entries (pure CURSED implementation)
- Supports up to 1,000 key-value pairs per map
- String equality comparison for key matching
- Handles map expansion by finding empty slots

### String Operations

- `string_length_enhanced(input tea) normie` - Unicode-aware string length calculation
- `string_char_at(str tea, index normie) normie` - Get character code at specific index
- `string_concat(a tea, b tea) tea` - Concatenate two strings
- `string_substring(str tea, start normie, end normie) tea` - Extract substring
- `string_to_byte_array(str tea) [normie]` - Convert string to byte array
- `runtime_strings_equal(a tea, b tea) lit` - String equality comparison

#### Implementation Details
- Character-by-character processing for string operations
- Bounds checking for all string access operations
- UTF-8 byte-level access through `get_string_byte_at`
- Efficient string concatenation and manipulation

### Memory Management

- `memory_allocate_bytes(size normie) normie` - Allocate memory block
- `memory_deallocate_bytes(pointer normie, size normie) lit` - Deallocate memory block
- `memory_copy_bytes(src normie, dest normie, size normie) lit` - Copy memory between blocks
- `memory_zero_bytes(pointer normie, size normie) lit` - Zero out memory block

#### Implementation Details
- Simulated memory allocation using timestamp-based pointers
- Size validation (0 < size <= 1MB)
- Memory tracking for allocation/deallocation pairs
- Safe parameter validation for all operations

### Time Operations

- `get_current_time_nanos() normie` - Get current time in nanoseconds
- `get_current_time_millis() normie` - Get current time in milliseconds  
- `time_elapsed_nanos(start_time normie) normie` - Calculate elapsed time

#### Implementation Details
- Incremental time counter for pure CURSED simulation
- Microsecond-precision time advancement
- Consistent time progression across function calls

### Performance Metrics

- `log_performance_metric(operation tea, duration normie) lit` - Log performance data
- `get_performance_stats() tea` - Get accumulated performance statistics

#### Implementation Details
- String-based performance log accumulation
- Metric format: "operation:duration_ns"
- Thread-safe logging through global state management

### Garbage Collection

- `trigger_gc_collection() lit` - Trigger garbage collection cycle
- `get_gc_statistics() tea` - Get comprehensive GC statistics
- `gc_mark_and_sweep() normie` - Perform mark-and-sweep collection
- Individual stat accessors: `gc_get_collection_count()`, `gc_get_memory_freed()`, `gc_get_live_object_count()`

#### Implementation Details
- Simulated mark-and-sweep algorithm
- 10% collection rate per GC cycle
- Comprehensive statistics tracking
- Memory usage estimation (64 bytes per object)

### Dynamic Function Calling

- `call_runtime_function(func_name tea, args [RuntimeValue]) RuntimeValue` - Dynamic function dispatch

#### Implementation Details
- Function registry with name-based dispatch
- Built-in support for common functions (`print`, `length`)
- Error handling for unknown functions
- Argument validation and type checking

## Value System

### Runtime Value Types

The module uses a `RuntimeValue` union type that can represent:
- `normie` - 32-bit integers
- `drip` - 64-bit floating point numbers  
- `tea` - Unicode strings
- `lit` - Boolean values (`based`/`cap`)
- `cringe` - Nil/null values

### Type Operations

- `runtime_value_create(value_data tea, value_type tea) RuntimeValue` - Create typed values
- `runtime_get_type(value RuntimeValue) tea` - Get type name
- `runtime_type_check(value RuntimeValue, expected_type tea) lit` - Type validation
- `runtime_convert_to_string(value RuntimeValue) tea` - String conversion
- `runtime_values_equal(a RuntimeValue, b RuntimeValue) lit` - Value equality

## Testing

Comprehensive tests are provided in `test_runtime_core.csd`:

```bash
# Test the runtime core module
cargo run --bin cursed stdlib/runtime_core/test_runtime_core.csd

# Test both interpretation and compilation modes
cargo run --bin cursed stdlib/runtime_core/test_runtime_core.csd
cargo run --bin cursed -- compile stdlib/runtime_core/test_runtime_core.csd
./test_runtime_core
```

### Test Coverage

- ✅ Basic value system operations
- ✅ Array operations with bounds checking
- ✅ Map operations with linear search
- ✅ String operations and manipulation
- ✅ Memory management simulation
- ✅ Time operations and elapsed calculation
- ✅ Performance metrics logging
- ✅ Garbage collection simulation
- ✅ Dynamic function calling
- ✅ Value comparison and equality
- ✅ Error handling and creation
- ✅ Helper function validation

## Architecture

### Pure CURSED Implementation

All operations are implemented in pure CURSED language without FFI dependencies:

- **No Rust FFI calls** - Eliminates external dependencies
- **Self-contained** - All logic implemented in CURSED
- **Simulation-based** - Uses mathematical simulation for system operations
- **Safe by design** - Bounds checking and parameter validation throughout

### Runtime Interface

The module provides both low-level operations and high-level runtime interfaces:

- **Low-level functions** - Direct array/map/string manipulation
- **Runtime interfaces** - Safe wrappers with additional validation
- **Error handling** - Comprehensive error creation and propagation
- **Performance tracking** - Built-in metrics collection

### Global State Management

Carefully managed global state for:
- Time counter progression
- Performance metrics accumulation  
- Garbage collection statistics
- Memory allocation tracking

## Integration

### Compiler Self-Hosting

This module enables CURSED compiler self-hosting by providing:

- **Value representation** - Runtime value types and operations
- **Memory management** - Allocation and deallocation interfaces
- **Type system support** - Type checking and conversion
- **Performance monitoring** - Execution metrics and profiling

### Runtime System

Integrates with the broader runtime system:

- **Interface definitions** - Standard runtime operation signatures
- **Error propagation** - Consistent error handling patterns
- **Resource management** - Memory and time resource tracking
- **Debugging support** - Comprehensive logging and statistics

## Future Enhancements

### Performance Optimizations

- Hash-based map implementations for O(1) operations
- More efficient string concatenation algorithms
- Advanced garbage collection strategies
- Memory pool allocation patterns

### Extended Operations

- Advanced string operations (regex, formatting)
- Complex number arithmetic  
- File I/O interfaces
- Network operation abstractions

### Integration Improvements

- Tighter integration with LLVM backend
- Enhanced debugging information generation
- Profile-guided optimization hooks
- Cross-platform compatibility layers

## Dependencies

- `testz` - Testing framework for validation
- CURSED language specifications from `specs/` directory
- Runtime value types and interfaces

## Compatibility

- ✅ Interpretation mode - Full compatibility
- ✅ Compilation mode - Full compatibility  
- ✅ Cross-platform - Pure CURSED implementation
- ✅ Self-hosting - Enables compiler self-hosting

This implementation represents a critical milestone in CURSED's journey toward full self-hosting capability by eliminating FFI dependencies in core runtime operations.
