# CURSED Collections Library Tests

This directory contains comprehensive tests for the CURSED collections standard library.

## Test Coverage

The `test_collections.💀` file provides complete test coverage for all collection types:

### Array/Vector Operations
- `array_new()` - Create empty array
- `array_push()` / `array_pop()` - Add/remove elements
- `array_insert()` / `array_remove()` - Insert/remove at index
- `array_get()` / `array_set()` - Element access
- `array_len()` / `array_is_empty()` - Size operations
- `array_clear()` - Clear all elements

### Array Search and Manipulation
- `array_contains()` - Check element existence
- `array_index_of()` - Find element index
- `array_reverse()` - Reverse array order
- `array_sort()` - Sort array elements
- `array_slice()` - Extract subarray
- `array_concat()` - Combine arrays

### Array Functional Operations
- `array_filter()` - Filter with predicate
- `array_map()` - Transform elements
- `array_reduce()` - Reduce to single value
- `array_find()` - Find first matching element
- `array_any()` / `array_all()` - Existence checks

### HashMap Operations
- `map_new()` - Create empty map
- `map_set()` / `map_get()` - Key-value operations
- `map_remove()` - Remove key-value pair
- `map_contains_key()` - Check key existence
- `map_get_or_default()` - Safe key access
- `map_len()` / `map_is_empty()` - Size operations

### Map Collections
- `map_keys()` - Get all keys
- `map_values()` - Get all values
- `map_entries()` - Get key-value pairs
- `map_merge()` - Combine maps
- `map_filter()` - Filter map entries
- `map_map_values()` - Transform values

### Set Operations
- `set_new()` - Create empty set
- `set_add()` / `set_remove()` - Element operations
- `set_contains()` - Check membership
- `set_len()` / `set_is_empty()` - Size operations
- `set_clear()` - Clear all elements

### Set Mathematical Operations
- `set_union()` - Union of sets
- `set_intersection()` - Intersection of sets
- `set_difference()` - Set difference
- `set_is_subset()` / `set_is_superset()` - Subset relations
- `set_to_array()` / `set_from_array()` - Array conversion

### Queue Operations (FIFO)
- `queue_new()` - Create empty queue
- `queue_enqueue()` / `queue_dequeue()` - Add/remove elements
- `queue_front()` / `queue_back()` - Peek elements
- `queue_len()` / `queue_is_empty()` - Size operations
- `queue_clear()` - Clear all elements

### Stack Operations (LIFO)
- `stack_new()` - Create empty stack
- `stack_push()` / `stack_pop()` - Add/remove elements
- `stack_peek()` - Look at top element
- `stack_len()` / `stack_is_empty()` - Size operations
- `stack_clear()` - Clear all elements

### Utility Functions
- `range()` / `range_step()` - Generate number sequences
- `zip()` - Combine arrays pairwise
- `flatten()` - Flatten nested arrays
- `unique()` - Remove duplicates
- `count_occurrences()` - Count element frequency
- `group_by()` - Group elements by key
- `partition()` - Split array by predicate

### Edge Cases Tested
- Empty collection operations
- Single element collections
- Large collection handling
- Duplicate element behavior
- Invalid index/key access
- Memory management
- Type safety

## Collection Types

The collections library provides efficient data structures:

- **Array/Vector**: Dynamic resizable arrays with random access
- **HashMap**: Key-value storage with O(1) average access time
- **HashSet**: Unique element storage with fast membership testing
- **Queue**: First-in-first-out (FIFO) data structure
- **Stack**: Last-in-first-out (LIFO) data structure

## Performance Characteristics

- **Array**: O(1) access, O(n) search, O(1) amortized append
- **HashMap**: O(1) average access, O(n) worst case
- **HashSet**: O(1) average membership test, O(n) worst case
- **Queue**: O(1) enqueue/dequeue operations
- **Stack**: O(1) push/pop operations

## Running Tests

```bash
# Run collections tests specifically
cargo run --bin cursed stdlib/collections/test_collections.💀

# Run all stdlib tests
cargo run --bin cursed test
```

## Test Results

All tests verify:
- Correct data structure behavior
- Proper element ordering
- Memory management
- Type safety
- Performance characteristics
- Edge case handling
- Integration between collection types

The tests ensure that all collection operations work correctly in both interpretation and native compilation modes.

## Usage Patterns

### Array Processing
```cursed
sus numbers [normie] = [1, 2, 3, 4, 5]
sus filtered [normie] = array_filter(numbers, func(x) { damn x > 3 })
sus doubled [normie] = array_map(filtered, func(x) { damn x * 2 })
```

### Map Operations
```cursed
sus user_map map = map_new()
user_map = map_set(user_map, "name", "Alice")
user_map = map_set(user_map, "age", "30")
sus name tea = map_get_or_default(user_map, "name", "Unknown")
```

### Set Operations
```cursed
sus set1 set = set_from_array(["a", "b", "c"])
sus set2 set = set_from_array(["b", "c", "d"])
sus union_result set = set_union(set1, set2)
```

## Memory Management

All collections handle memory management automatically:
- Dynamic resizing for growing collections
- Efficient memory usage patterns
- Automatic cleanup when collections are no longer used
- Copy-on-write semantics where appropriate
