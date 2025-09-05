# Collections Core Module

A comprehensive collections library implemented in pure CURSED, providing fundamental data structures for all stdlib modules and applications.

## Overview

The collections_core module provides efficient, memory-safe implementations of essential data structures without any FFI dependencies. All implementations use pure CURSED language features and follow modern algorithms for optimal performance.

## Data Structures

### Dynamic Vector/Array
- **Type**: `Vector`
- **Features**: Automatic growth, random access, efficient append operations
- **Growth Strategy**: 2x capacity expansion when full
- **Time Complexity**: O(1) amortized append, O(1) access, O(n) insert/remove

```cursed
sus vec *Vector = vector_new()
vector_push(vec, 42)
vector_push(vec, 100)
sus value normie = vector_get(vec, 0)  # Returns 42
vector_remove(vec, 0)                  # Remove first element
vector_free(vec)
```

### Linked Lists
- **Types**: `LinkedList` (single or double linked)
- **Features**: Efficient insertion/deletion, no memory reallocation
- **Time Complexity**: O(1) front operations, O(1) back operations (double linked)

```cursed
# Single linked list
sus list *LinkedList = list_new(cap)
list_push_front(list, 10)
list_push_back(list, 20)
sus value normie = list_remove_front(list)
list_free(list)

# Double linked list
sus dlist *LinkedList = list_new(based)
list_push_front(dlist, 30)
list_push_back(dlist, 40)
list_free(dlist)
```

### Hash Map
- **Type**: `HashMap`
- **Features**: String keys, collision handling via chaining, dynamic resizing
- **Load Factor**: 0.75 maximum for optimal performance
- **Time Complexity**: O(1) average operations, O(n) worst case

```cursed
sus map *HashMap = hashmap_new(16)
hashmap_put(map, "key1", 100)
hashmap_put(map, "key2", 200)
sus value normie = hashmap_get(map, "key1")  # Returns 100
hashmap_remove(map, "key2")
hashmap_free(map)
```

### Set
- **Type**: `Set`
- **Features**: Unique string elements, built on HashMap
- **Time Complexity**: O(1) average add/remove/contains operations

```cursed
sus set *Set = set_new()
set_add(set, "apple")
set_add(set, "banana")
sus exists lit = set_contains(set, "apple")  # Returns based
set_remove(set, "banana")
set_free(set)
```

### Binary Search Tree (BST) / AVL Tree
- **Type**: `BST`
- **Features**: Ordered traversal, optional AVL self-balancing
- **Balancing**: AVL rotations for O(log n) guaranteed height
- **Time Complexity**: O(log n) average/worst case (AVL), O(n) worst case (unbalanced BST)

```cursed
# Regular BST
sus bst *BST = tree_new(cap)
tree_insert(bst, 50)
tree_insert(bst, 30)
tree_insert(bst, 70)
sus found lit = tree_contains(bst, 50)  # Returns based
tree_free(bst)

# AVL Tree (self-balancing)
sus avl *BST = tree_new(based)
tree_insert(avl, 10)
tree_insert(avl, 20)
tree_insert(avl, 30)  # Automatic rotation
tree_free(avl)
```

### Heap
- **Type**: `Heap`
- **Features**: Min-heap or max-heap, efficient priority operations
- **Implementation**: Binary heap with array storage
- **Time Complexity**: O(log n) insert/extract, O(1) peek

```cursed
# Max heap
sus max_heap *Heap = heap_new(10, based)
heap_insert(max_heap, 30)
heap_insert(max_heap, 10)
heap_insert(max_heap, 20)
sus max normie = heap_extract(max_heap)  # Returns 30
heap_free(max_heap)

# Min heap
sus min_heap *Heap = heap_new(10, cap)
heap_insert(min_heap, 30)
heap_insert(min_heap, 10)
heap_insert(min_heap, 20)
sus min normie = heap_extract(min_heap)  # Returns 10
heap_free(min_heap)
```

### Queue
- **Type**: `Queue`
- **Features**: FIFO operations, circular buffer implementation
- **Time Complexity**: O(1) enqueue/dequeue operations

```cursed
sus queue *Queue = queue_new(5)
queue_enqueue(queue, 10)
queue_enqueue(queue, 20)
sus value normie = queue_dequeue(queue)  # Returns 10
sus front normie = queue_peek(queue)     # Returns 20
queue_free(queue)
```

### Stack
- **Type**: `Stack`
- **Features**: LIFO operations, array-based implementation
- **Time Complexity**: O(1) push/pop operations

```cursed
sus stack *Stack = stack_new(5)
stack_push(stack, 10)
stack_push(stack, 20)
sus value normie = stack_pop(stack)   # Returns 20
sus top normie = stack_peek(stack)    # Returns 10
stack_free(stack)
```

### Priority Queue
- **Type**: `PriorityQueue`
- **Features**: Built on heap, configurable min/max priority
- **Time Complexity**: O(log n) enqueue/dequeue, O(1) peek

```cursed
# Max priority queue
sus pq *PriorityQueue = priority_queue_new(10, based)
priority_queue_enqueue(pq, 30)
priority_queue_enqueue(pq, 10)
priority_queue_enqueue(pq, 20)
sus highest normie = priority_queue_dequeue(pq)  # Returns 30
priority_queue_free(pq)
```

## Implementation Features

### Memory Management
- All data structures handle their own memory allocation/deallocation
- No memory leaks when proper cleanup functions are called
- Efficient memory usage with minimal overhead

### Thread Safety
- Data structures are not thread-safe by default (for performance)
- For concurrent access, wrap operations in appropriate synchronization

### Error Handling
- Functions return appropriate error values (0, cap, cringe) on failure
- Bounds checking on array-based structures
- Graceful handling of empty collections

### Performance Characteristics
- Optimized implementations using standard algorithms
- Minimal function call overhead
- Cache-friendly memory layouts where possible

## Usage Patterns

### Iterator Pattern
Most collections support iteration through direct access or callback functions:

```cursed
# Vector iteration
bestie i := 0; i < vec.size; i++ {
    sus value normie = vector_get(vec, i)
    vibez.spill("Value: %d", value)
}
```

### Memory Management Pattern
Always pair creation with cleanup:

```cursed
sus collection *DataStructure = data_structure_new(...)
# Use collection...
data_structure_free(collection)
```

### Error Checking Pattern
Check return values for error conditions:

```cursed
lowkey !vector_push(vec, value) {
    vibez.spill("Failed to add value to vector")
}
```

## Testing

The module includes comprehensive tests covering:
- Basic functionality for all data structures
- Edge cases and error conditions
- Stress tests with large datasets
- Memory management verification
- Performance characteristics
- Integration scenarios

Run tests with:
```bash
cargo run --bin cursed stdlib/collections_core/test_collections_core.💀
```

## Dependencies

- **testz**: Testing framework (test file only)
- **Pure CURSED**: No FFI dependencies
- **Runtime**: Basic memory allocation functions (malloc/free)

## Performance Notes

### Space Complexity
- Vector: O(n) space with growth overhead
- LinkedList: O(n) space, no growth overhead
- HashMap: O(n) space with load factor overhead
- Set: O(n) space (same as HashMap)
- BST/AVL: O(n) space, no overhead
- Heap: O(n) space, no overhead
- Queue: O(n) space with fixed capacity
- Stack: O(n) space with fixed capacity
- PriorityQueue: O(n) space (same as Heap)

### Time Complexity Summary
| Operation | Vector | List | HashMap | Set | BST | AVL | Heap | Queue | Stack | PQ |
|-----------|--------|------|---------|-----|-----|-----|------|-------|-------|----| 
| Insert    | O(1)*  | O(1) | O(1)*   | O(1)* | O(n) | O(log n) | O(log n) | O(1) | O(1) | O(log n) |
| Remove    | O(n)   | O(1) | O(1)*   | O(1)* | O(n) | O(log n) | O(log n) | O(1) | O(1) | O(log n) |
| Search    | O(n)   | O(n) | O(1)*   | O(1)* | O(n) | O(log n) | N/A      | N/A   | N/A   | O(1) peek |
| Access    | O(1)   | O(n) | O(1)*   | O(1)* | O(log n) | O(log n) | O(1) peek | O(1) peek | O(1) peek | O(1) peek |

*Amortized or average case

## Future Enhancements

Potential future additions:
- Iterator interfaces for consistent traversal
- Generic type support when language feature is available
- Concurrent data structures with built-in synchronization
- Specialized collections (e.g., circular buffers, B-trees)
- Memory pools for allocation optimization
- Serialization/deserialization support

## Examples

See `test_collections_core.💀` for comprehensive usage examples of all data structures and their integration patterns.
