# Advanced Collections Module - CURSED Language

## Overview

The Advanced Collections Module provides enterprise-grade, production-ready data structures implemented natively in CURSED. This module includes advanced tree structures, priority queues, and thread-safe concurrent collections optimized for high-performance applications.

## Features

### 🌳 Advanced Tree Structures
- **B-Trees**: Self-balancing search trees optimized for systems with large branching factors
- **AVL Trees**: Height-balanced binary search trees with guaranteed O(log n) operations
- **Red-Black Trees**: Balanced binary search trees with efficient insertion and deletion

### 🔄 Priority Data Structures
- **Priority Queue (Max Heap)**: Efficient priority-based element management
- **Min Heap**: Minimum-priority queue implementation
- **Indexed Priority Queue**: Priority queue with fast priority updates

### 🧵 Concurrent Collections
- **Lock-Free Stack**: Thread-safe stack using atomic operations
- **Lock-Free Queue**: Thread-safe FIFO queue with lock-free algorithms
- **Concurrent HashMap**: Segmented hash map with fine-grained locking
- **Work-Stealing Queue**: High-performance work distribution queue
- **Concurrent Set**: Thread-safe set operations

### ⚡ Performance Optimizations
- **Memory Pool Management**: Efficient memory allocation and deallocation
- **Garbage Collection Integration**: Optimized for CURSED's GC system
- **Cache-Friendly Layouts**: Data structures optimized for CPU cache performance
- **SIMD Operations**: Vectorized operations where applicable

## Quick Start

```cursed
yeet "advanced_collections"

// Create and use a B-Tree
sus btree BTree = btree_new(5)
btree = btree_insert(btree, "key1", "value1")
sus value tea = btree_search(btree, "key1")

// Create and use a Priority Queue
sus pq PriorityQueue = priority_queue_new()
pq = priority_queue_insert(pq, "high priority task", 10)
pq = priority_queue_insert(pq, "low priority task", 1)
sus next_task tea = priority_queue_extract_max(pq)

// Create and use concurrent collections
sus concurrent_map ConcurrentHashMapAdvanced = concurrent_hashmap_advanced_new(16)
concurrent_hashmap_advanced_insert(concurrent_map, "key", "value")
sus result tea = concurrent_hashmap_advanced_get(concurrent_map, "key")
```

## Data Structures

### B-Tree

B-Trees are self-balancing search trees that maintain sorted data and allow searches, sequential access, insertions, and deletions in logarithmic time.

**Key Features:**
- Configurable minimum degree (branching factor)
- Automatic node splitting and merging
- Optimized for disk-based storage systems
- Range query support

**Usage:**
```cursed
sus tree BTree = btree_new(5)  // Min degree of 5
tree = btree_insert(tree, "apple", "fruit")
tree = btree_insert(tree, "banana", "yellow")
sus fruit tea = btree_search(tree, "apple")
```

**Time Complexity:**
- Search: O(log n)
- Insert: O(log n)
- Delete: O(log n)
- Space: O(n)

### AVL Tree

AVL Trees are height-balanced binary search trees where the height difference between left and right subtrees is at most 1.

**Key Features:**
- Automatic rebalancing through rotations
- Guaranteed O(log n) operations
- Height-balanced property maintained
- Efficient for frequent lookups

**Usage:**
```cursed
sus tree AVLTree = avl_new()
tree = avl_insert(tree, "delta", "d")
tree = avl_insert(tree, "beta", "b")
tree = avl_insert(tree, "alpha", "a")  // Triggers rotation
sus value tea = avl_search(tree, "beta")
```

**Time Complexity:**
- Search: O(log n)
- Insert: O(log n)
- Delete: O(log n)
- Space: O(n)

### Priority Queue (Max Heap)

Priority Queues maintain elements in priority order, allowing efficient access to the highest priority element.

**Key Features:**
- Max heap implementation (highest priority first)
- Dynamic resizing
- Efficient heapify operations
- Priority-based task scheduling

**Usage:**
```cursed
sus pq PriorityQueue = priority_queue_new()
pq = priority_queue_insert(pq, "urgent", 10)
pq = priority_queue_insert(pq, "normal", 5)
pq = priority_queue_insert(pq, "low", 1)

sus highest tea = priority_queue_extract_max(pq)  // "urgent"
sus next tea = priority_queue_extract_max(pq)     // "normal"
```

**Time Complexity:**
- Insert: O(log n)
- Extract Max: O(log n)
- Peek: O(1)
- Space: O(n)

### Lock-Free Stack

Thread-safe stack implementation using atomic operations without locks.

**Key Features:**
- Lock-free algorithm using compare-and-swap
- Thread-safe operations
- High performance under contention
- ABA problem resistant

**Usage:**
```cursed
sus stack LockFreeStack = lockfree_stack_new()
lockfree_stack_push(stack, "item1")
lockfree_stack_push(stack, "item2")
sus item tea = lockfree_stack_pop(stack)  // "item2"
```

**Time Complexity:**
- Push: O(1) amortized
- Pop: O(1) amortized
- Size: O(1)
- Space: O(n)

### Lock-Free Queue

Thread-safe FIFO queue implementation using lock-free algorithms.

**Key Features:**
- Lock-free Michael & Scott algorithm
- Thread-safe enqueue/dequeue operations
- Memory-efficient design
- Suitable for producer-consumer patterns

**Usage:**
```cursed
sus queue LockFreeQueue = lockfree_queue_new()
lockfree_queue_enqueue(queue, "first")
lockfree_queue_enqueue(queue, "second")
sus item tea = lockfree_queue_dequeue(queue)  // "first"
```

**Time Complexity:**
- Enqueue: O(1) amortized
- Dequeue: O(1) amortized
- Size: O(1)
- Space: O(n)

### Concurrent HashMap

Thread-safe hash map with segmented locking for high concurrency.

**Key Features:**
- Segmented locking strategy
- Configurable number of segments
- Fine-grained concurrency control
- Automatic load balancing

**Usage:**
```cursed
sus chm ConcurrentHashMapAdvanced = concurrent_hashmap_advanced_new(16)
concurrent_hashmap_advanced_insert(chm, "user:123", "user_data")
concurrent_hashmap_advanced_insert(chm, "session:456", "session_data")
sus data tea = concurrent_hashmap_advanced_get(chm, "user:123")
```

**Time Complexity:**
- Insert: O(1) average, O(n) worst case
- Get: O(1) average, O(n) worst case
- Remove: O(1) average, O(n) worst case
- Space: O(n)

### Work-Stealing Queue

High-performance queue designed for work distribution in parallel systems.

**Key Features:**
- Owner can push/pop from one end (LIFO)
- Thieves can steal from the other end (FIFO)
- Lock-free steal operations
- Ideal for task parallelism

**Usage:**
```cursed
sus wsq WorkStealingQueue = work_stealing_queue_new(100)
work_stealing_queue_push(wsq, "task1")    // Owner adds work
work_stealing_queue_push(wsq, "task2")
sus task tea = work_stealing_queue_pop(wsq)    // Owner takes work (LIFO)
sus stolen tea = work_stealing_queue_steal(wsq) // Thief steals work (FIFO)
```

**Time Complexity:**
- Push: O(1)
- Pop: O(1)
- Steal: O(1)
- Space: O(capacity)

## Performance Characteristics

### Memory Usage

| Data Structure | Memory Overhead | Cache Performance | GC Pressure |
|---------------|-----------------|-------------------|-------------|
| B-Tree | Low | Excellent | Low |
| AVL Tree | Medium | Good | Medium |
| Priority Queue | Low | Excellent | Low |
| Lock-Free Stack | Low | Good | Low |
| Lock-Free Queue | Medium | Good | Medium |
| Concurrent HashMap | Medium | Good | Medium |
| Work-Stealing Queue | Low | Excellent | Low |

### Concurrency Performance

| Data Structure | Thread Safety | Scalability | Contention Handling |
|---------------|---------------|-------------|-------------------|
| B-Tree | Single-threaded | N/A | N/A |
| AVL Tree | Single-threaded | N/A | N/A |
| Priority Queue | Single-threaded | N/A | N/A |
| Lock-Free Stack | Thread-safe | Excellent | Lock-free |
| Lock-Free Queue | Thread-safe | Excellent | Lock-free |
| Concurrent HashMap | Thread-safe | Very Good | Fine-grained locks |
| Work-Stealing Queue | Thread-safe | Excellent | Lock-free |

## Testing

### Running Tests

```bash
# Test all advanced collections
cargo run --bin cursed stdlib/collections/test_advanced_collections.💀

# Test concurrent collections
cargo run --bin cursed stdlib/collections/test_concurrent_collections.💀

# Run performance benchmarks
cargo run --bin cursed stdlib/collections/performance_benchmarks.💀

# Test both interpretation and compilation modes
cargo run --bin cursed stdlib/collections/test_advanced_collections.💀
cargo run --bin cursed -- compile stdlib/collections/test_advanced_collections.💀
./test_advanced_collections
```

### Test Coverage

The test suite includes:
- **Basic Operations**: CRUD operations for all data structures
- **Edge Cases**: Empty collections, single elements, capacity limits
- **Concurrent Operations**: Multi-threaded scenarios
- **Performance Tests**: Benchmark comparisons
- **Memory Tests**: Memory usage and garbage collection
- **Integration Tests**: Real-world usage scenarios

## Best Practices

### Choosing the Right Data Structure

1. **General Key-Value Storage**: Use `HashMap` or `ConcurrentHashMap`
2. **Sorted Data with Range Queries**: Use `BTree`
3. **Balanced Tree Operations**: Use `AVLTree`
4. **Priority-Based Processing**: Use `PriorityQueue`
5. **Thread-Safe Stack Operations**: Use `LockFreeStack`
6. **Producer-Consumer Patterns**: Use `LockFreeQueue`
7. **Task Distribution**: Use `WorkStealingQueue`

### Performance Tips

1. **B-Tree Degree**: Choose degree based on expected data size and access patterns
2. **HashMap Segments**: Use more segments for higher concurrency
3. **Queue Capacity**: Size work-stealing queues based on expected workload
4. **Memory Pools**: Consider pre-allocating for frequently used collections
5. **Batch Operations**: Group operations when possible to reduce overhead

### Memory Management

1. **Garbage Collection**: All collections integrate with CURSED's GC
2. **Memory Pools**: Use memory pools for high-frequency allocations
3. **Cleanup**: Explicitly clear collections when no longer needed
4. **Reference Management**: Be aware of circular references in tree structures

## Advanced Features

### Custom Comparators

```cursed
// Custom string comparison for case-insensitive operations
slay custom_string_compare(s1 tea, s2 tea) normie {
    // Convert to lowercase and compare
    // Implementation would handle case conversion
    damn string_compare_ignore_case(s1, s2)
}
```

### Memory Pool Integration

```cursed
// Use memory pools for high-performance scenarios
slay use_memory_pool() {
    memory_usage_report()
    trigger_gc()
    memory_pool_info()
}
```

### Atomic Operations

```cursed
// Low-level atomic operations for custom concurrent structures
sus counter normie = 0
sus ptr *normie = &counter
atomic_fetch_add_int(ptr, 1)
sus value normie = atomic_load_int(ptr)
```

## Integration with CURSED Runtime

The Advanced Collections module integrates seamlessly with CURSED's runtime system:

- **Garbage Collection**: Automatic memory management
- **Type System**: Strong typing with compile-time checks
- **Error Handling**: Graceful error recovery
- **Performance Monitoring**: Built-in performance metrics
- **Thread Safety**: Proper synchronization primitives

## Future Enhancements

Planned improvements include:
- **B+ Trees**: Optimized for range queries
- **Skip Lists**: Probabilistic balanced data structure
- **Bloom Filters**: Space-efficient probabilistic data structure
- **Consistent Hashing**: For distributed systems
- **Memory-Mapped Collections**: For large datasets
- **SIMD Optimizations**: Vectorized operations for bulk operations

## Contributing

When contributing to the Advanced Collections module:

1. **Follow CURSED conventions**: Use proper naming and syntax
2. **Write comprehensive tests**: Include edge cases and performance tests
3. **Document performance characteristics**: Include time/space complexity
4. **Test both modes**: Verify functionality in interpretation and compilation
5. **Consider thread safety**: Document concurrency guarantees
6. **Memory efficiency**: Optimize for CURSED's garbage collector

## License

This module is part of the CURSED programming language and follows the same licensing terms.
