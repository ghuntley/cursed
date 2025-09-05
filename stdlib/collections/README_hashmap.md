# CURSED HashMap Implementation

A native hash map implementation written in CURSED for key-value storage. This implementation provides O(1) average-case performance for basic operations and serves as a replacement for Rust's `std::collections::HashMap`.

## Features

- **Generic Key-Value Storage**: Stores string keys and string values
- **Collision Resolution**: Uses open addressing with linear probing
- **Dynamic Resizing**: Automatically resizes when load factor exceeds 0.75
- **Memory Efficient**: Compact representation with minimal overhead
- **Full API Coverage**: All essential HashMap operations implemented

## Architecture

### Core Components

1. **HashMapEntry Structure**: Individual key-value pairs with metadata
2. **HashMap Structure**: Main container with entries array and statistics
3. **Hash Functions**: String and integer hashing with collision handling
4. **Resize Logic**: Automatic growth when load factor threshold is reached

### Hash Function Design

The implementation uses the DJB2 hash function for strings:
```cursed
hash = 5381
for each character in key:
    hash = ((hash << 5) + hash) + character
```

For integers, it uses a simple multiplicative hash with bit mixing:
```cursed
hash = key ^ (key >> 16)
hash = hash * 0x45d9f3b
hash = hash ^ (hash >> 16)
```

### Collision Resolution

Open addressing with linear probing is used for collision resolution:
- When a collision occurs, the algorithm searches for the next available slot
- Deleted entries are marked with `is_deleted` flag for proper handling
- Load factor is maintained below 0.75 to minimize clustering

## API Reference

### Construction

```cursed
// Create new empty HashMap with default capacity (16)
sus map HashMap = hashmap_new()

// Create HashMap with specific initial capacity
sus map HashMap = hashmap_with_capacity(32)
```

### Core Operations

```cursed
// Insert key-value pair
map = hashmap_insert(map, "key", "value")

// Get value by key (returns "" if not found)
sus value tea = hashmap_get(map, "key")

// Check if key exists
sus exists lit = hashmap_contains_key(map, "key")

// Remove key-value pair
map = hashmap_remove(map, "key")

// Clear all entries
map = hashmap_clear(map)
```

### Information

```cursed
// Get number of entries
sus size normie = hashmap_len(map)

// Check if empty
sus empty lit = hashmap_is_empty(map)

// Get all keys as array
sus keys []tea = hashmap_keys(map)

// Get all values as array
sus values []tea = hashmap_values(map)
```

### Debug and Utility

```cursed
// Print debug information
hashmap_print_debug(map)

// Run example usage
hashmap_example()
```

## Performance Characteristics

| Operation | Average Case | Worst Case | Notes |
|-----------|--------------|------------|-------|
| Insert    | O(1)         | O(n)       | Amortized due to resize |
| Get       | O(1)         | O(n)       | Linear probing clusters |
| Remove    | O(1)         | O(n)       | Tombstone marking |
| Contains  | O(1)         | O(n)       | Same as Get |
| Keys      | O(n)         | O(n)       | Must scan all entries |
| Values    | O(n)         | O(n)       | Must scan all entries |

### Memory Usage

- **Fixed Overhead**: ~32 bytes per HashMap instance
- **Entry Overhead**: ~24 bytes per key-value pair
- **Load Factor**: Maintained at 75% to balance space and time
- **Resize Strategy**: Doubles capacity when threshold exceeded

## Usage Examples

### Basic Operations

```cursed
yeet "stdlib/collections/hashmap.💀"

slay main() {
    sus map HashMap = hashmap_new()
    
    // Insert some data
    map = hashmap_insert(map, "name", "CURSED")
    map = hashmap_insert(map, "version", "1.0")
    map = hashmap_insert(map, "language", "Systems")
    
    // Query data
    vibez.spill("Name: " + hashmap_get(map, "name"))
    vibez.spill("Version: " + hashmap_get(map, "version"))
    
    // Check existence
    lowkey hashmap_contains_key(map, "author") {
        vibez.spill("Author: " + hashmap_get(map, "author"))
    } highkey {
        vibez.spill("Author not found")
    }
    
    // Get statistics
    vibez.spill("Total entries: " + tea(hashmap_len(map)))
    vibez.spill("Is empty: " + tea(hashmap_is_empty(map)))
}
```

### Iteration Over Keys and Values

```cursed
slay iterate_example() {
    sus map HashMap = hashmap_new()
    
    map = hashmap_insert(map, "apple", "red")
    map = hashmap_insert(map, "banana", "yellow")
    map = hashmap_insert(map, "grape", "purple")
    
    // Iterate over all keys
    sus keys []tea = hashmap_keys(map)
    sus i normie = 0
    bestie i < hashmap_len(map) {
        sus key tea = keys[i]
        sus value tea = hashmap_get(map, key)
        vibez.spill(key + " -> " + value)
        i = i + 1
    }
}
```

### Configuration Management

```cursed
slay config_example() {
    sus config HashMap = hashmap_new()
    
    // Load configuration
    config = hashmap_insert(config, "database_url", "localhost:5432")
    config = hashmap_insert(config, "max_connections", "100")
    config = hashmap_insert(config, "timeout", "30")
    config = hashmap_insert(config, "debug", "false")
    
    // Use configuration
    sus db_url tea = hashmap_get(config, "database_url")
    sus max_conn tea = hashmap_get(config, "max_connections")
    
    vibez.spill("Connecting to: " + db_url)
    vibez.spill("Max connections: " + max_conn)
}
```

## Testing

Run the comprehensive test suite:

```bash
# Run HashMap tests
cargo run --bin cursed stdlib/collections/test_hashmap.💀

# Run all collections tests
cargo run --bin cursed stdlib/collections/test_collections.💀
```

The test suite covers:
- Basic operations (insert, get, remove, clear)
- Edge cases (empty map, single item, many items)
- Hash function correctness
- Collision handling
- Dynamic resizing
- Array operations (keys, values)

## Implementation Notes

### String Operations

The implementation assumes runtime support for:
- `string_length(s)` - Get string length
- `string_char_at(s, index)` - Get character at index
- `tea(value)` - Convert value to string

### Memory Management

- Uses CURSED's native array allocation with `make([]Type, size)`
- Relies on CURSED's garbage collector for cleanup
- No explicit memory deallocation required

### Type Safety

- Currently supports string keys and string values
- Future versions could be extended to support generic types
- Type assertions could be added for runtime type checking

### Thread Safety

- This implementation is not thread-safe
- For concurrent usage, external synchronization would be required
- Future versions could add optional locking mechanisms

## Limitations

1. **Key Type**: Only string keys are supported
2. **Value Type**: Only string values are supported
3. **Hash Quality**: Simple hash function may have clustering issues
4. **Memory**: No memory pool optimization for frequent allocations
5. **Iteration Order**: No guaranteed iteration order

## Future Enhancements

1. **Generic Types**: Support for arbitrary key and value types
2. **Better Hash Functions**: More sophisticated hash functions for better distribution
3. **Robin Hood Hashing**: Improved collision resolution strategy
4. **Memory Pool**: Optimized memory allocation for better performance
5. **Concurrent HashMap**: Thread-safe variant with fine-grained locking
6. **Serialization**: Support for saving/loading HashMap state

## Integration with CURSED Compiler

This HashMap implementation is designed to replace Rust's `std::collections::HashMap` usage throughout the CURSED compiler codebase. Key integration points:

1. **Symbol Tables**: Replace Rust HashMap for identifier resolution
2. **Type Checking**: Store type information and constraints
3. **Code Generation**: Cache generated code and metadata
4. **Optimization**: Track optimization opportunities and results
5. **Debug Information**: Store debugging symbols and source mappings

The native implementation provides better integration with CURSED's type system and memory management while maintaining familiar API semantics.
