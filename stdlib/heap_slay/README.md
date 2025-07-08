# heap_slay - Pure CURSED Heap Data Structure Module

A comprehensive heap data structure implementation in pure CURSED, providing both min-heap and max-heap operations along with priority queue functionality.

## Features

### Core Heap Operations
- **Min/Max Heap Creation**: `heap_min_new()`, `heap_max_new()`
- **Insertion**: `heap_insert()` with automatic heap property maintenance
- **Extraction**: `heap_extract()` removes and returns root element
- **Peek**: `heap_peek()` returns root element without removal
- **Size Management**: `heap_size()`, `heap_is_empty()`, `heap_is_full()`

### Advanced Heap Operations
- **Heap Construction**: `heap_build_from_array()` builds heap from existing array
- **Heap Validation**: `heap_validate()` verifies heap property
- **Heap Sort**: `heap_sort()` sorts array using heap sort algorithm
- **Kth Largest Element**: `heap_kth_largest()` finds kth largest element efficiently

### Priority Queue
- **Priority Queue Creation**: `pq_new()` creates priority queue using min-heap
- **Enqueue/Dequeue**: `pq_enqueue()`, `pq_dequeue()` for priority-based operations
- **Peek**: `pq_peek()` examines highest priority element
- **Status**: `pq_is_empty()`, `pq_size()` for queue management

### Utility Functions
- **Index Calculations**: `heap_parent()`, `heap_left_child()`, `heap_right_child()`
- **Element Comparison**: `heap_compare()` handles min/max heap logic
- **Element Swapping**: `heap_swap()` for internal heap operations

## Usage Examples

### Basic Min Heap Operations
```cursed
# Create min heap with capacity 10
sus h *Heap = heap_min_new(10)

# Insert elements
heap_insert(h, 5)
heap_insert(h, 3)
heap_insert(h, 8)
heap_insert(h, 1)

# Peek at minimum (root)
sus min_val normie = heap_peek(h)  # Returns 1

# Extract minimum
sus extracted normie = heap_extract(h)  # Returns 1
```

### Basic Max Heap Operations
```cursed
# Create max heap with capacity 10
sus h *Heap = heap_max_new(10)

# Insert elements
heap_insert(h, 5)
heap_insert(h, 3)
heap_insert(h, 8)
heap_insert(h, 1)

# Peek at maximum (root)
sus max_val normie = heap_peek(h)  # Returns 8

# Extract maximum
sus extracted normie = heap_extract(h)  # Returns 8
```

### Priority Queue Operations
```cursed
# Create priority queue (min-heap based)
sus pq *PriorityQueue = pq_new(10)

# Enqueue elements with priorities
pq_enqueue(pq, 5)  # Priority 5
pq_enqueue(pq, 2)  # Priority 2 (higher priority)
pq_enqueue(pq, 8)  # Priority 8

# Dequeue highest priority element
sus high_priority normie = pq_dequeue(pq)  # Returns 2
```

### Build Heap from Array
```cursed
# Create array and heap
sus arr []normie = [5, 3, 8, 1, 9, 2, 7]
sus h *Heap = heap_min_new(10)

# Build heap from array
heap_build_from_array(h, arr, 7)

# Validate heap property
sus is_valid lit = heap_validate(h)  # Returns based (true)
```

### Heap Sort
```cursed
# Sort array using heap sort
sus arr []normie = [64, 34, 25, 12, 22, 11, 90]
heap_sort(arr, 7)
# Array is now sorted in ascending order
```

### Find Kth Largest Element
```cursed
# Find 3rd largest element
sus arr []normie = [3, 1, 4, 1, 5, 9, 2, 6, 5]
sus third_largest normie = heap_kth_largest(arr, 9, 3)  # Returns 5
```

## Data Structures

### Heap Structure
```cursed
struct Heap {
    data []normie      # Array to store heap elements
    size normie        # Current number of elements
    capacity normie    # Maximum capacity
    is_min_heap lit    # based for min-heap, cap for max-heap
}
```

### Priority Queue Structure
```cursed
struct PriorityQueue {
    heap *Heap         # Underlying heap structure
}
```

## Algorithm Complexity

| Operation | Time Complexity | Space Complexity |
|-----------|----------------|------------------|
| Insert | O(log n) | O(1) |
| Extract | O(log n) | O(1) |
| Peek | O(1) | O(1) |
| Build Heap | O(n) | O(1) |
| Heap Sort | O(n log n) | O(1) |
| Kth Largest | O(n log k) | O(k) |

## Error Handling

The module provides robust error handling:
- **Capacity Limits**: Operations fail gracefully when heap is full
- **Empty Heap**: Returns -1 for operations on empty heaps
- **Invalid Parameters**: Validates input parameters and returns error codes
- **Heap Property**: Maintains heap property through all operations

## Testing

Run the comprehensive test suite:
```bash
# Test in interpretation mode
cargo run --bin cursed stdlib/heap_slay/test_heap_slay.csd

# Test in compilation mode
cargo run --bin cursed -- compile stdlib/heap_slay/test_heap_slay.csd
./test_heap_slay
```

The test suite covers:
- Min and max heap operations
- Priority queue functionality
- Edge cases and error conditions
- Complex operation sequences
- Heap validation and property maintenance
- Performance characteristics

## Implementation Notes

- **Pure CURSED**: No FFI dependencies, implemented entirely in CURSED
- **Memory Efficient**: Uses array-based implementation for cache efficiency
- **Type Safe**: Leverages CURSED's type system for safe operations
- **Flexible**: Supports both min-heap and max-heap with single implementation
- **Validated**: Comprehensive test coverage ensures correctness

## Use Cases

- **Priority Queues**: Task scheduling, event processing
- **Sorting**: Heap sort implementation
- **Graph Algorithms**: Dijkstra's algorithm, Prim's MST
- **Data Stream Processing**: Finding top-k elements
- **Memory Management**: Heap-based allocation strategies
- **Real-time Systems**: Priority-based resource allocation

## Module Dependencies

- `testz`: Testing framework (test files only)
- No external dependencies for core functionality
