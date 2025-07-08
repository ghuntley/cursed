# Sort Slay Module - High-Performance Sorting Algorithms

The `sort_slay` module provides comprehensive, high-performance sorting algorithms implemented in pure CURSED. This module offers various sorting strategies optimized for different use cases and data types.

## Features

- **Multiple Sorting Algorithms**: QuickSort, MergeSort for different performance characteristics
- **Type-Specific Optimizations**: Specialized implementations for integers, strings, and floats
- **Stable and Unstable Sorting**: Choose between stability and performance
- **Search Operations**: Binary search with bounds checking
- **Utility Functions**: Merge, partition, and element selection operations
- **Pure CURSED Implementation**: No external dependencies or FFI bridges

## Functions

### Basic Sorting

#### `sort_ints(arr []normie) []normie`
Sorts an array of integers using optimized QuickSort algorithm.

```cursed
sus numbers []normie = [3, 1, 4, 1, 5, 9, 2, 6]
sus sorted []normie = sort_ints(numbers)
// Result: [1, 1, 2, 3, 4, 5, 6, 9]
```

#### `sort_strings(arr []tea) []tea`
Sorts an array of strings lexicographically using QuickSort.

```cursed
sus words []tea = ["zebra", "apple", "banana", "cherry"]
sus sorted []tea = sort_strings(words)
// Result: ["apple", "banana", "cherry", "zebra"]
```

#### `sort_floats(arr []meal) []meal`
Sorts an array of floating-point numbers using QuickSort.

```cursed
sus numbers []meal = [3.14, 2.71, 1.41, 1.73]
sus sorted []meal = sort_floats(numbers)
// Result: [1.41, 1.73, 2.71, 3.14]
```

### Advanced Sorting

#### `sort_stable(arr []normie) []normie`
Performs stable sorting using MergeSort, preserving relative order of equal elements.

```cursed
sus data []normie = [3, 1, 4, 1, 5]
sus sorted []normie = sort_stable(data)
// Result: [1, 1, 3, 4, 5] (stable ordering maintained)
```

#### `sort_unstable(arr []normie) []normie`
Performs unstable sorting using QuickSort for maximum performance.

```cursed
sus data []normie = [3, 1, 4, 1, 5]
sus sorted []normie = sort_unstable(data)
// Result: [1, 1, 3, 4, 5] (faster but not stable)
```

#### `sort_reverse(arr []normie) []normie`
Sorts an array in descending order.

```cursed
sus numbers []normie = [1, 3, 2, 5, 4]
sus reversed []normie = sort_reverse(numbers)
// Result: [5, 4, 3, 2, 1]
```

### Utility Functions

#### `is_sorted(arr []normie) lit`
Checks if an array is sorted in ascending order.

```cursed
sus sorted []normie = [1, 2, 3, 4, 5]
sus unsorted []normie = [3, 1, 4, 1, 5]
assert_true(is_sorted(sorted))
assert_false(is_sorted(unsorted))
```

#### `partition(arr []normie, pivot normie) normie`
Partitions an array around a pivot element, returning the pivot's final position.

```cursed
sus data []normie = [3, 1, 4, 1, 5, 9, 2, 6]
sus pivot_pos normie = partition(data, 4)
// Elements less than pivot are moved to the left
```

#### `quick_select(arr []normie, k normie) normie`
Finds the k-th smallest element in an array using QuickSelect algorithm.

```cursed
sus data []normie = [3, 1, 4, 1, 5, 9, 2, 6]
sus third_smallest normie = quick_select(data, 2)  // k=2 for 3rd smallest
// Result: 2
```

#### `merge(arr1 []normie, arr2 []normie) []normie`
Merges two sorted arrays into a single sorted array.

```cursed
sus left []normie = [1, 3, 5, 7]
sus right []normie = [2, 4, 6, 8]
sus merged []normie = merge(left, right)
// Result: [1, 2, 3, 4, 5, 6, 7, 8]
```

### Search Operations

#### `binary_search(arr []normie, target normie) normie`
Performs binary search on a sorted array, returning the index of the target element.

```cursed
sus sorted []normie = [1, 2, 3, 4, 5, 6, 7, 8, 9]
sus index normie = binary_search(sorted, 5)
// Result: 4 (0-based index)
// Returns -1 if not found
```

#### `lower_bound(arr []normie, target normie) normie`
Finds the first position where the target element could be inserted to maintain sorted order.

```cursed
sus data []normie = [1, 2, 2, 3, 3, 3, 4, 5]
sus pos normie = lower_bound(data, 3)
// Result: 3 (first position of value 3)
```

#### `upper_bound(arr []normie, target normie) normie`
Finds the last position where the target element could be inserted to maintain sorted order.

```cursed
sus data []normie = [1, 2, 2, 3, 3, 3, 4, 5]
sus pos normie = upper_bound(data, 3)
// Result: 6 (position after last 3)
```

## Performance Characteristics

### QuickSort (Unstable)
- **Average Case**: O(n log n)
- **Worst Case**: O(n²)
- **Best Case**: O(n log n)
- **Space Complexity**: O(log n)
- **Stability**: No

### MergeSort (Stable)
- **Average Case**: O(n log n)
- **Worst Case**: O(n log n)
- **Best Case**: O(n log n)
- **Space Complexity**: O(n)
- **Stability**: Yes

### QuickSelect
- **Average Case**: O(n)
- **Worst Case**: O(n²)
- **Space Complexity**: O(log n)

### Binary Search
- **Time Complexity**: O(log n)
- **Space Complexity**: O(1)
- **Prerequisite**: Array must be sorted

## Usage Examples

### Basic Sorting Example

```cursed
yeet "sort_slay"

slay demo_basic_sorting() {
    sus numbers []normie = [64, 34, 25, 12, 22, 11, 90]
    
    vibez.spill("Original array:")
    print_array(numbers)
    
    sus sorted []normie = sort_ints(numbers)
    vibez.spill("Sorted array:")
    print_array(sorted)
    
    if is_sorted(sorted) {
        vibez.spill("Array is correctly sorted!")
    }
}
```

### Advanced Operations Example

```cursed
yeet "sort_slay"

slay demo_advanced_operations() {
    sus data []normie = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5]
    
    // Find the 5th smallest element
    sus fifth_smallest normie = quick_select(data, 4)
    vibez.spill("5th smallest element:", fifth_smallest)
    
    // Merge two sorted arrays
    sus arr1 []normie = sort_ints([1, 3, 5, 7])
    sus arr2 []normie = sort_ints([2, 4, 6, 8])
    sus merged []normie = merge(arr1, arr2)
    
    // Binary search in sorted array
    sus target normie = 5
    sus index normie = binary_search(merged, target)
    if index != -1 {
        vibez.spill("Found", target, "at index", index)
    }
}
```

### Performance Comparison Example

```cursed
yeet "sort_slay"

slay demo_performance_comparison() {
    sus large_data []normie = generate_random_array(1000)
    
    // Test unstable sort (faster)
    sus unstable_sorted []normie = sort_unstable(large_data)
    vibez.spill("Unstable sort completed")
    
    // Test stable sort (preserves order)
    sus stable_sorted []normie = sort_stable(large_data)
    vibez.spill("Stable sort completed")
    
    // Both should be sorted
    assert_true(is_sorted(unstable_sorted))
    assert_true(is_sorted(stable_sorted))
}
```

## Best Practices

1. **Choose the Right Algorithm**:
   - Use `sort_unstable()` for maximum performance when stability isn't required
   - Use `sort_stable()` when you need to preserve the relative order of equal elements
   - Use `sort_reverse()` when you need descending order

2. **Type-Specific Sorting**:
   - Use `sort_ints()` for integer arrays
   - Use `sort_strings()` for string arrays
   - Use `sort_floats()` for floating-point arrays

3. **Search Operations**:
   - Ensure arrays are sorted before using `binary_search()`
   - Use `lower_bound()` and `upper_bound()` for range queries
   - Use `quick_select()` when you only need the k-th element, not the entire sorted array

4. **Memory Considerations**:
   - All sorting functions return new arrays, preserving the original
   - For in-place operations, consider implementing custom variants
   - Stable sort uses more memory due to temporary arrays

## Testing

The module includes comprehensive tests covering:
- Basic sorting functionality for all data types
- Edge cases (empty arrays, single elements, duplicates)
- Performance with larger datasets
- Search operations accuracy
- Stability verification for stable sorts

Run tests with:
```bash
# Test interpretation mode
cargo run --bin cursed stdlib/sort_slay/test_sort_slay.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/sort_slay/test_sort_slay.csd
./test_sort_slay
```

## Implementation Details

- **Pure CURSED**: No external dependencies or FFI bridges
- **Optimized Algorithms**: Efficient implementations of classic algorithms
- **Memory Safe**: Proper bounds checking and memory management
- **Type Safe**: Leverages CURSED's type system for safety
- **Tested**: Comprehensive test suite with edge case coverage

## Future Enhancements

- **Parallel Sorting**: Multi-threaded sorting for large datasets
- **Custom Comparators**: User-defined comparison functions
- **Specialized Algorithms**: Radix sort, counting sort for specific use cases
- **Streaming Sort**: Sorting of data streams larger than memory
- **Benchmarking**: Performance measurement and comparison tools
