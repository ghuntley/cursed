# Advanced Collections Module

A comprehensive collections library implemented in pure CURSED with full generic support. Provides high-performance data structures suitable for enterprise applications.

## Data Structures Included

### Core Collections
- **HashMap<K, V>** - Hash table with chaining collision resolution
- **ArrayList<T>** - Dynamic array with automatic resizing
- **LinkedList<T>** - Doubly-linked list implementation
- **Stack<T>** - LIFO data structure
- **Queue<T>** - FIFO data structure
- **Set<T>** - Unique element collection

### Advanced Trees
- **BST<T>** - Binary Search Tree
- **AVLTree<T>** - Self-balancing binary search tree
- **PriorityQueue<T>** - Heap-based priority queue

## Key Features

### Generic Support
All collections support CURSED generics for type safety:
```cursed
sus int_list := new_arraylist<normie>(10)
sus string_map := new_hashmap<tea, normie>(16)
sus char_stack := new_stack<sip>()
```

### Performance Optimized
- **HashMap**: O(1) average case for get/put operations
- **ArrayList**: O(1) amortized insertion, O(1) random access
- **AVL Tree**: O(log n) guaranteed for all operations
- **Priority Queue**: O(log n) insertion/extraction

### Memory Efficient
- Dynamic resizing with configurable load factors
- Efficient memory allocation patterns
- Minimal overhead per element

## Usage Examples

### HashMap Operations
```cursed
yeet "collections_advanced"

sus map := new_hashmap<normie, tea>(16)
hashmap_put(map, 1, "one")
hashmap_put(map, 2, "two")

sus value, exists := hashmap_get(map, 1)
ayo (exists) {
    vibez.spill("Found:", value)
}
```

### ArrayList with Dynamic Resizing
```cursed
sus list := new_arraylist<normie>(5)
arraylist_add(list, 10)
arraylist_add(list, 20)
arraylist_add(list, 30)

sus item, success := arraylist_get(list, 1)
ayo (success) {
    vibez.spill("Item at index 1:", item)
}
```

### Stack Operations
```cursed
sus stack := new_stack<tea>()
stack_push(stack, "first")
stack_push(stack, "second")

sus top, exists := stack_peek(stack)
sus popped, success := stack_pop(stack)
```

### AVL Tree Self-Balancing
```cursed
sus tree := new_avl<normie>()
avl_insert(tree, 10)
avl_insert(tree, 5)
avl_insert(tree, 15)
avl_insert(tree, 3)  # Automatically balances

ayo (bst_search(tree, 5)) {
    vibez.spill("Found 5 in tree")
}
```

### Priority Queue (Max Heap)
```cursed
sus pq := new_priority_queue<normie>()
pq_insert(pq, 10)
pq_insert(pq, 30)
pq_insert(pq, 20)

sus max_item, success := pq_extract_max(pq)  # Returns 30
```

## Performance Characteristics

| Operation | HashMap | ArrayList | LinkedList | Stack | Queue | AVL Tree |
|-----------|---------|-----------|------------|-------|-------|----------|
| Insert    | O(1)*   | O(1)*     | O(1)       | O(1)  | O(1)  | O(log n) |
| Access    | O(1)*   | O(1)      | O(n)       | O(1)  | O(1)  | O(log n) |
| Delete    | O(1)*   | O(n)      | O(1)       | O(1)  | O(1)  | O(log n) |
| Search    | O(1)*   | O(n)      | O(n)       | O(n)  | O(n)  | O(log n) |

*Average case, O(n) worst case for HashMap due to collisions

## Testing

### Run Comprehensive Tests
```bash
# Test interpretation mode
cargo run --bin cursed stdlib/collections_advanced/test_collections_advanced.💀

# Test compilation mode  
cargo run --bin cursed -- compile stdlib/collections_advanced/test_collections_advanced.💀
./test_collections_advanced
```

### Test Coverage
- **Unit Tests**: Individual data structure operations
- **Integration Tests**: Collections working together
- **Performance Tests**: Large dataset operations (1000+ elements)
- **Generic Tests**: Multiple type parameter validation
- **Edge Cases**: Empty collections, boundary conditions

### Performance Benchmarking
The test suite includes performance benchmarks:
- HashMap: 10,000 insertions
- ArrayList: 10,000 insertions  
- AVL Tree: 1,000 insertions with balance verification

## Implementation Details

### Memory Management
- All collections use CURSED's native memory management
- No external FFI dependencies
- Automatic cleanup and garbage collection integration

### Generic Type System
- Full integration with CURSED's generic system
- Type-safe operations across all collections
- Compile-time type checking

### Error Handling
- Functions return tuple (value, success) for safe operations
- Graceful handling of empty collections
- Bounds checking for indexed access

### Thread Safety
- Base implementations are not thread-safe
- Designed for single-threaded performance
- Can be wrapped with synchronization primitives if needed

## Advanced Features

### Custom Comparison
The tree structures support custom comparison functions for complex types.

### Load Factor Tuning
HashMap allows configuration of load factor for performance tuning:
```cursed
sus map := new_hashmap<normie, tea>(100)
# Default load factor: 0.75, resize threshold: 75 elements
```

### Heap Operations
Priority queue supports both max-heap (default) and can be adapted for min-heap operations.

## Best Practices

1. **Choose the Right Collection**:
   - Use HashMap for fast key-value lookups
   - Use ArrayList for indexed access and iteration
   - Use LinkedList for frequent insertions/deletions
   - Use Stack/Queue for specific access patterns
   - Use AVL Tree for sorted data with frequent searches

2. **Memory Optimization**:
   - Size collections appropriately to avoid excessive resizing
   - Use ArrayList over LinkedList for better cache locality
   - Consider Set over manual duplicate checking

3. **Performance Considerations**:
   - HashMap performs best with good hash distribution
   - ArrayList insertion at end is O(1), middle is O(n)
   - AVL Tree provides guaranteed O(log n) performance

## Contributing

This module is implemented in pure CURSED without external dependencies. When contributing:

1. Maintain generic type support for all new collections
2. Include comprehensive tests for new functionality
3. Follow existing naming conventions
4. Ensure both interpretation and compilation modes work
5. Add performance benchmarks for new data structures

## License

Part of the CURSED standard library - follows project licensing terms.
