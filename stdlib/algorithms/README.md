# Algorithms Module

## Why This Module Exists

The `algorithms` module provides high-performance, production-ready implementations of fundamental computer science algorithms that every serious application needs. Rather than forcing developers to implement their own sorting, searching, and data structure algorithms (often incorrectly), this module provides battle-tested, optimized implementations that handle edge cases, maintain performance guarantees, and follow established algorithmic patterns.

The module exists because:
- **Performance Criticality**: Sorting and searching operations appear in 80% of applications and poor algorithm choice can destroy performance
- **Correctness Assurance**: Complex algorithms like QuickSort have subtle edge cases that novice implementations often miss
- **Memory Efficiency**: Proper implementations use optimal memory patterns and avoid unnecessary allocations
- **Algorithmic Diversity**: Different use cases need different algorithms (stable vs unstable sorts, memory vs speed tradeoffs)

## Why Testing Is Critical

Algorithm testing is absolutely essential because:
- **Edge Case Complexity**: Algorithms must handle empty arrays, single elements, duplicate values, already-sorted data, reverse-sorted data, and randomized inputs
- **Performance Regressions**: Subtle implementation changes can cause O(n²) worst-case behavior in algorithms that should be O(n log n)
- **Memory Safety**: In-place algorithms manipulate array indices and can easily cause buffer overflows without proper bounds checking
- **Stability Requirements**: Some applications require stable sorts that preserve the original order of equal elements
- **Correctness Under Load**: Algorithms must maintain correctness under high-concurrency scenarios and large datasets

## Implementation Rationale

### Key Design Decisions:

**1. Hybrid Algorithm Approach**
- QuickSort with Introsort fallback prevents O(n²) worst-case performance
- Insertion sort for small arrays (< 16 elements) leverages cache locality
- Heap sort as guaranteed O(n log n) fallback maintains performance promises

**2. Generic Type System Integration**
- Template-based implementation works with any comparable type
- Custom comparator support enables complex sorting criteria
- Zero-cost abstractions ensure no performance penalty for generics

**3. Memory Management Strategy**
- In-place algorithms minimize memory allocation
- Arena allocator integration for temporary storage when needed
- Stack-friendly recursion with iterative fallbacks for deep recursion

**4. Concurrency Safety**
- Immutable input guarantee: original arrays never modified unless explicitly requested
- Thread-safe implementations for parallel sorting operations
- Lock-free data structures for concurrent access patterns

## API Reference

### Sorting Functions

#### `sort<T>(array: []T, comparator?: slay(T, T) bool)`
**Purpose**: General-purpose stable sorting with optimal performance across all input distributions.
**Time Complexity**: O(n log n) average and worst-case
**Space Complexity**: O(log n)
**Stability**: Stable

```cursed
sus numbers []drip = [5, 2, 8, 1, 9, 3]
algorithms.sort(numbers)
# numbers is now [1, 2, 3, 5, 8, 9]

# Custom comparator for descending order
algorithms.sort(numbers, slay(a drip, b drip) bool { damn a > b })
```

#### `quicksort<T>(array: []T, comparator?: slay(T, T) bool)`
**Purpose**: High-performance unstable sort optimized for random data
**Time Complexity**: O(n log n) average, O(n log n) worst-case (introsort hybrid)
**Space Complexity**: O(log n)
**Stability**: Unstable

#### `mergesort<T>(array: []T, comparator?: slay(T, T) bool)`
**Purpose**: Guaranteed stable sort with predictable performance
**Time Complexity**: O(n log n) guaranteed
**Space Complexity**: O(n)
**Stability**: Stable

### Search Functions

#### `binary_search<T>(array: []T, target: T) drip`
**Purpose**: Logarithmic search in sorted arrays
**Time Complexity**: O(log n)
**Precondition**: Array must be sorted
**Returns**: Index if found, -1 if not found

```cursed
sus sorted []drip = [1, 3, 5, 7, 9, 11, 13]
sus index drip = algorithms.binary_search(sorted, 7)
# index is 3
```

#### `linear_search<T>(array: []T, target: T) drip`
**Purpose**: Simple search for unsorted arrays
**Time Complexity**: O(n)
**Returns**: Index if found, -1 if not found

### Data Structure Operations

#### `heap_push<T>(heap: []T, value: T, comparator?: slay(T, T) bool)`
**Purpose**: Insert element maintaining heap property
**Time Complexity**: O(log n)

#### `heap_pop<T>(heap: []T, comparator?: slay(T, T) bool) T`
**Purpose**: Remove and return maximum/minimum element
**Time Complexity**: O(log n)

## Usage Examples

### Simple Sorting
```cursed
yeet "algorithms"

# Basic number sorting
sus numbers []drip = [64, 34, 25, 12, 22, 11, 90]
algorithms.sort(numbers)
vibez.spill("Sorted:", numbers)  # [11, 12, 22, 25, 34, 64, 90]
```

### Custom Comparators
```cursed
# Sort strings by length
sus words []tea = ["apple", "hi", "algorithm", "go"]
algorithms.sort(words, slay(a tea, b tea) bool {
    damn a.len() < b.len()
})
# Result: ["go", "hi", "apple", "algorithm"]
```

### Advanced Data Structures
```cursed
# Priority queue using heap operations
sus priorities []drip = []
algorithms.heap_push(priorities, 5)
algorithms.heap_push(priorities, 2)
algorithms.heap_push(priorities, 8)
algorithms.heap_push(priorities, 1)

sus highest drip = algorithms.heap_pop(priorities)  # 8
sus next drip = algorithms.heap_pop(priorities)     # 5
```

### Performance-Critical Applications
```cursed
# Large dataset sorting with performance monitoring
yeet "timez"

sus large_data []drip = generate_random_array(100000)
sus start_time = timez.now()

algorithms.quicksort(large_data)  # Fastest for random data

sus duration = timez.since(start_time)
vibez.spill("Sorted 100k elements in:", duration, "ms")
```

## Performance Considerations

### Algorithm Selection Guide

**Use `sort()` when**:
- You need stable sorting (preserves equal element order)
- Input characteristics are unknown
- General-purpose sorting is required

**Use `quicksort()` when**:
- Maximum performance is critical
- Data is mostly randomized
- Stability is not required

**Use `mergesort()` when**:
- Stability is required
- Predictable O(n log n) performance is needed
- Working with linked lists or external sorting

### Performance Optimization Tips

1. **Pre-allocate Arrays**: Avoid dynamic resizing during algorithm execution
2. **Cache-Friendly Access**: Access array elements sequentially when possible  
3. **Custom Comparators**: Keep comparator functions simple and fast
4. **Memory Locality**: Use algorithms appropriate for your data size and memory hierarchy
5. **Benchmark Your Use Case**: Different algorithms excel with different data distributions

### Memory Usage Patterns

- **In-place algorithms**: QuickSort, HeapSort use O(log n) space
- **External algorithms**: MergeSort uses O(n) additional space
- **Arena integration**: Temporary allocations use arena allocators for zero-leak guarantee

## Security Considerations

### Algorithmic Complexity Attacks

**Threat**: Adversarial input can trigger worst-case O(n²) behavior in naive QuickSort implementations
**Mitigation**: This module uses Introsort hybrid that automatically switches to HeapSort when recursion depth exceeds 2*log(n)

### Input Validation

**Threat**: Buffer overflow through malicious array indices
**Mitigation**: All array access uses bounds-checked operations with automatic panics on overflow

### Deterministic Behavior

**Threat**: Timing attacks based on sorting performance variations
**Mitigation**: Use `mergesort()` for cryptographic applications where timing consistency is required

### Memory Safety

**Threat**: Use-after-free or double-free in algorithm implementations
**Mitigation**: All algorithms integrated with CURSED's memory safety system and arena allocators

## Thread Safety

All functions are thread-safe for concurrent reads of the same array, but concurrent modification requires external synchronization. Use the `sync` module's locks when multiple threads need to modify the same data structure.

```cursed
yeet "sync"

sus shared_array []drip = [3, 1, 4, 1, 5]
sus mutex = sync.Mutex.new()

# Thread-safe sorting
mutex.lock()
algorithms.sort(shared_array)
mutex.unlock()
```
