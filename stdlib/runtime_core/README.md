# CURSED Runtime Core Library

High-performance data structures and utilities for the CURSED compiler runtime, implemented entirely in pure CURSED.

## Overview

The Runtime Core library provides essential data structures and utilities that the CURSED compiler uses internally. These implementations are optimized for compiler workloads and use no external FFI dependencies.

## Core Data Structures

### RuntimeVec<T> - Dynamic Array
Generic dynamic array implementation similar to Zig's ArrayList:

```cursed
sus vec RuntimeVec<normie> = RuntimeVec_new<normie>()
vec = RuntimeVec_push(vec, 42)
vec = RuntimeVec_push(vec, 100)
sus value normie = RuntimeVec_get(vec, 0)  // 42
sus length normie = RuntimeVec_len(vec)    // 2
```

**Features:**
- Automatic capacity growth (starts at 4, doubles on overflow)
- Generic type support for any CURSED type
- Bounds checking with safe zero-value returns
- Memory-efficient contiguous storage

### RuntimeHashMap<K, V> - Hash Table
Generic hash map for symbol tables and fast lookups:

```cursed
sus map RuntimeHashMap<tea, normie> = RuntimeHashMap_new<tea, normie>()
map = RuntimeHashMap_insert(map, "variable_count", 42)
(value, found) := RuntimeHashMap_get(map, "variable_count")
```

**Features:**
- Linear probing collision resolution
- String-optimized hash function (djb2 algorithm)
- Generic key-value support
- O(1) average case performance

### RuntimeStringBuilder - Efficient String Construction
Optimized string building for compiler output generation:

```cursed
sus sb RuntimeStringBuilder = RuntimeStringBuilder_new()
sb = RuntimeStringBuilder_append(sb, "function ")
sb = RuntimeStringBuilder_append(sb, function_name)
sb = RuntimeStringBuilder_append_char(sb, '(')
sus code tea = RuntimeStringBuilder_to_string(sb)
```

**Features:**
- Avoids O(n²) string concatenation
- Tracks total length for efficiency
- Supports both string and character appending
- Memory-efficient part storage

## Utility Structures

### RuntimeStack<T> - Stack Data Structure
Generic stack for runtime state management:

```cursed
sus stack RuntimeStack<tea> = RuntimeStack_new<tea>()
stack = RuntimeStack_push(stack, "function_scope")
(current_scope, success) := RuntimeStack_pop(stack)
```

### RuntimeError - Structured Error Handling
Comprehensive error information for compiler diagnostics:

```cursed
sus err RuntimeError = RuntimeError_with_source(
    E_SYNTAX_ERROR, 
    "Unexpected token", 
    "main.csd", 
    42
)
sus error_msg tea = RuntimeError_to_string(err)
```

### RuntimeMemoryPool - Block Allocator
Efficient memory allocation for temporary compiler data:

```cursed
sus pool RuntimeMemoryPool = RuntimeMemoryPool_new(4096)
sus buffer []byte = RuntimeMemoryPool_allocate(pool, 256)
```

## Performance Characteristics

| Operation | Time Complexity | Space Complexity |
|-----------|----------------|------------------|
| Vec push | O(1) amortized | O(n) |
| Vec get | O(1) | - |
| HashMap insert | O(1) average | O(n) |
| HashMap lookup | O(1) average | - |
| StringBuilder append | O(1) amortized | O(n) |
| Stack push/pop | O(1) | O(n) |

## Compiler Integration

The Runtime Core library is designed specifically for compiler workloads:

- **Symbol Tables**: Use RuntimeHashMap for variable/function lookups
- **Token Storage**: Use RuntimeVec for dynamic token arrays
- **Code Generation**: Use RuntimeStringBuilder for efficient output
- **Error Tracking**: Use RuntimeError for diagnostic information
- **Scope Management**: Use RuntimeStack for nested scopes

## Testing

Run the comprehensive test suite:

```bash
./cursed-unified stdlib/runtime_core/test_runtime_core.csd
```

## Implementation Notes

- All data structures use pure CURSED with no FFI dependencies
- Generic implementations work with any CURSED type
- Memory-safe with bounds checking and error handling
- Optimized for typical compiler access patterns
- Compatible with both interpretation and compilation modes

## API Reference

### RuntimeVec<T>
- `RuntimeVec_new<T>() RuntimeVec<T>`
- `RuntimeVec_with_capacity<T>(normie) RuntimeVec<T>`
- `RuntimeVec_push<T>(RuntimeVec<T>, T) RuntimeVec<T>`
- `RuntimeVec_get<T>(RuntimeVec<T>, normie) T`
- `RuntimeVec_len<T>(RuntimeVec<T>) normie`
- `RuntimeVec_is_empty<T>(RuntimeVec<T>) lit`

### RuntimeHashMap<K, V>
- `RuntimeHashMap_new<K, V>() RuntimeHashMap<K, V>`
- `RuntimeHashMap_insert<K, V>(RuntimeHashMap<K, V>, K, V) RuntimeHashMap<K, V>`
- `RuntimeHashMap_get<K, V>(RuntimeHashMap<K, V>, K) (V, lit)`
- `RuntimeHashMap_contains<K, V>(RuntimeHashMap<K, V>, K) lit`
- `RuntimeHashMap_size<K, V>(RuntimeHashMap<K, V>) normie`

### RuntimeStringBuilder
- `RuntimeStringBuilder_new() RuntimeStringBuilder`
- `RuntimeStringBuilder_append(RuntimeStringBuilder, tea) RuntimeStringBuilder`
- `RuntimeStringBuilder_append_char(RuntimeStringBuilder, sip) RuntimeStringBuilder`
- `RuntimeStringBuilder_to_string(RuntimeStringBuilder) tea`
- `RuntimeStringBuilder_len(RuntimeStringBuilder) normie`

### RuntimeStack<T>
- `RuntimeStack_new<T>() RuntimeStack<T>`
- `RuntimeStack_push<T>(RuntimeStack<T>, T) RuntimeStack<T>`
- `RuntimeStack_pop<T>(RuntimeStack<T>) (T, lit)`
- `RuntimeStack_peek<T>(RuntimeStack<T>) (T, lit)`
- `RuntimeStack_is_empty<T>(RuntimeStack<T>) lit`
- `RuntimeStack_size<T>(RuntimeStack<T>) normie`

### RuntimeError
- `RuntimeError_new(normie, tea) RuntimeError`
- `RuntimeError_with_source(normie, tea, tea, normie) RuntimeError`
- `RuntimeError_to_string(RuntimeError) tea`
