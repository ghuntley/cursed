# arrayz - CURSED Array Operations Module

Pure CURSED implementation for comprehensive array manipulation operations.

## Features

### Core Array Creation
- `array_new()` - Create empty array
- `array_fill(size, value)` - Create array filled with value
- `array_range(start, end)` - Create array of numbers in range

### Basic Operations
- `array_length(arr)` - Get array length
- `array_get(arr, index)` - Get element at index
- `array_set(arr, index, value)` - Set element at index
- `array_push(arr, value)` - Add element to end
- `array_pop(arr)` - Remove and return last element
- `array_insert(arr, index, value)` - Insert element at index
- `array_remove(arr, index)` - Remove element at index

### Search Operations
- `array_find(arr, value)` - Find first occurrence index
- `array_contains(arr, value)` - Check if value exists
- `array_count(arr, value)` - Count occurrences

### Manipulation
- `array_reverse(arr)` - Reverse array order
- `array_slice(arr, start, end)` - Extract sub-array
- `array_concat(arr1, arr2)` - Combine two arrays
- `array_join(arr, separator)` - Join elements into string

### Sorting
- `array_sort_strings(arr)` - Sort string array
- `array_sort_numbers(arr)` - Sort numeric array

### Set Operations
- `array_unique(arr)` - Remove duplicates
- `array_intersection(arr1, arr2)` - Common elements
- `array_difference(arr1, arr2)` - Elements in arr1 not in arr2
- `array_union(arr1, arr2)` - All unique elements from both

### Functional Operations
- `array_filter(arr, predicate)` - Filter elements
- `array_map(arr, mapper)` - Transform elements
- `array_reduce(arr, initial, reducer)` - Reduce to single value

### Validation
- `array_all(arr, predicate)` - Check if all elements match
- `array_any(arr, predicate)` - Check if any element matches
- `array_none(arr, predicate)` - Check if no elements match

### Utilities
- `array_chunk(arr, size)` - Split into chunks
- `array_flatten(nested_arr)` - Flatten nested arrays
- `array_zip(arr1, arr2)` - Combine into pairs

## Usage Example

```cursed
yeet "arrayz"

# Create and manipulate arrays
sus numbers [normie] = arrayz.array_range(1, 6)  # [1, 2, 3, 4, 5]
sus doubled [normie] = arrayz.array_map(numbers, slay(x normie) normie { damn x * 2 })
sus sum normie = arrayz.array_sum_numbers(doubled)

# String operations
sus words [tea] = ["hello", "world", "test"]
sus sentence tea = arrayz.array_join(words, " ")
sus filtered [tea] = arrayz.array_filter(words, slay(w tea) lit { damn len(w) > 4 })
```

## Dependencies
- Pure CURSED implementation - no FFI dependencies
- Uses core CURSED language features only

## Testing
Run tests with: `./zig-out/bin/cursed stdlib/arrayz/test_arrayz.csd`
