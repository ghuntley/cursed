# complex_module

Advanced stdlib functionality testing module that demonstrates complex import dependencies, data structures, and algorithmic operations. Serves as a comprehensive integration test for CURSED's module system.

## Overview

The `complex_module` module provides:
- Complex data structure implementations
- Advanced search and transformation algorithms
- Multi-level processing workflows
- String processing with module dependencies
- Comprehensive validation and testing

## Core Components

### ComplexData Structure

A sophisticated data structure that combines arrays, maps, and computed values:

```cursed
squad ComplexData {
    spill values []normie
    spill metadata map[tea]tea
    spill computed_hash normie
}
```

#### Factory Functions

#### `complex_data_new() -> ComplexData`
Creates a new empty ComplexData instance with initialized fields.

**Returns:** Empty ComplexData structure ready for use

#### `complex_data_add_value(data: ComplexData, value: normie) -> ComplexData`
Adds a value to the data structure and recomputes the hash.

**Parameters:**
- `data`: The ComplexData instance to modify
- `value`: Integer value to add

**Returns:** Updated ComplexData with new value and recomputed hash

#### `complex_data_set_metadata(data: ComplexData, key: tea, value: tea) -> ComplexData`
Sets metadata key-value pairs in the data structure.

**Parameters:**
- `data`: The ComplexData instance to modify
- `key`: Metadata key
- `value`: Metadata value

**Returns:** Updated ComplexData with new metadata

### Algorithmic Operations

#### `compute_hash(values: []normie) -> normie`
Computes a hash value for an array of integers using the djb2 algorithm.

**Parameters:**
- `values`: Array of integers to hash

**Returns:** 32-bit hash value

**Algorithm:**
```
hash = 5381
for each value in values:
    hash = ((hash << 5) + hash) + value
```

#### `complex_search(data: ComplexData, target: normie) -> lit`
Performs binary search on the values array to find a target value.

**Parameters:**
- `data`: ComplexData instance to search
- `target`: Value to find

**Returns:** `based` if found, `cringe` if not found

**Requirements:** Values array must be sorted for correct results

#### `complex_transform(input: []normie) -> []normie`
Applies a mathematical transformation to each element in the input array.

**Parameters:**
- `input`: Array of integers to transform

**Returns:** Array with transformed values

**Transformation:** `f(x) = x² + x`

### String Processing

#### `complex_string_process(input: tea) -> tea`
Processes strings using functionality from the `string_simple` module.

**Parameters:**
- `input`: String to process

**Returns:** Processed string with "processed_" prefix

## Usage Examples

### Basic Data Structure Operations

```cursed
yeet "complex_module"

// Create and populate data structure
sus data ComplexData = complex_data_new()
data = complex_data_add_value(data, 42)
data = complex_data_add_value(data, 84)
data = complex_data_set_metadata(data, "type", "test_data")

vibez.spill("Hash: " + string(data.computed_hash))
vibez.spill("Values: " + string(len(data.values)))
```

### Search Operations

```cursed
// Search for values (requires sorted array)
sus found lit = complex_search(data, 42)
lowkey found {
    vibez.spill("Value 42 found in data structure")
} yikes {
    vibez.spill("Value 42 not found")
}
```

### Transformation Operations

```cursed
// Transform an array of values
sus input []normie = []normie{1, 2, 3, 4, 5}
sus output []normie = complex_transform(input)

bestie i := 0; i < len(output); i = i + 1 {
    vibez.spill("Transformed: " + string(input[i]) + " -> " + string(output[i]))
}
// Output: 1->2, 2->6, 3->12, 4->20, 5->30
```

### String Processing

```cursed
// Process strings with module dependencies
sus result tea = complex_string_process("hello")
vibez.spill(result) // "processed_hello"
```

## Testing and Validation

### Comprehensive Test Suite

#### `complex_module_validate() -> lit`
Runs a comprehensive validation suite testing all module functionality.

**Test Coverage:**
- Data structure creation and manipulation
- Hash computation verification
- Search algorithm correctness
- Transformation function accuracy
- String processing integration
- Module dependency validation

**Returns:** `based` if all tests pass

### Test Categories

1. **Data Structure Tests**
   - Creation and initialization
   - Value addition and metadata setting
   - Hash computation verification

2. **Algorithm Tests**
   - Binary search with various inputs
   - Edge cases (empty arrays, not found)
   - Transformation function accuracy

3. **Integration Tests**
   - Cross-module string processing
   - Dependency loading verification
   - End-to-end workflows

## Dependencies

The module demonstrates complex dependency management:

```cursed
yeet "testz"          // Testing framework
yeet "collections"    // Data structure utilities
yeet "string_simple"  // String processing
yeet "math"          // Mathematical operations
```

### Dependency Graph

```
complex_module
├── testz (testing)
├── collections (data structures)
├── string_simple (string processing)
└── math (mathematical operations)
    └── [transitive dependencies]
```

## Performance Characteristics

### Time Complexity
- **Hash computation**: O(n) where n is array length
- **Binary search**: O(log n) where n is array length
- **Array transformation**: O(n) where n is array length
- **String processing**: O(m) where m is string length

### Space Complexity
- **Data structure**: O(n + k) where n is values, k is metadata keys
- **Transformation**: O(n) for output array
- **Search**: O(1) auxiliary space

## Advanced Features

### Hash Algorithm Details

The module uses the djb2 hash algorithm for reliable hash computation:

```
Initial: hash = 5381
For each byte: hash = hash * 33 + byte
```

This provides good distribution for typical integer sequences.

### Binary Search Implementation

Implements classic binary search with proper bounds checking:

```
while left <= right:
    mid = (left + right) / 2
    if array[mid] == target: return found
    if array[mid] < target: left = mid + 1
    else: right = mid - 1
```

### Metadata System

Flexible key-value metadata system supporting arbitrary string pairs:
- Type information
- Processing flags
- User-defined attributes
- Debugging information

## Error Handling

The module includes comprehensive error handling:
- Bounds checking for array operations
- Null pointer validation
- Invalid input detection
- Graceful degradation

## Testing

```bash
# Run module tests
zig build test
./zig-out/bin/cursed-zig stdlib/complex_module/test_complex_module.csd

# Run validation
./zig-out/bin/cursed-zig -c "
yeet 'complex_module'
complex_module_validate()
"
```

## Integration Examples

### With Collections Module

```cursed
yeet "complex_module"
yeet "collections"

// Create complex data with collections integration
sus data ComplexData = complex_data_new()
sus list []normie = collections.create_list()
// ... integrate collections operations
```

### With Math Module

```cursed
yeet "complex_module"
yeet "math"

// Enhanced transformations with math functions
slay enhanced_transform(input []normie) []normie {
    sus result []normie = complex_transform(input)
    bestie i := 0; i < len(result); i = i + 1 {
        result[i] = math.abs(result[i])
    }
    damn result
}
```

## Architecture

The module follows a layered architecture:

1. **Data Layer**: Core data structures and storage
2. **Algorithm Layer**: Search, transform, and processing functions
3. **Integration Layer**: Cross-module functionality
4. **Validation Layer**: Testing and verification

This design enables both standalone usage and complex integration scenarios.
