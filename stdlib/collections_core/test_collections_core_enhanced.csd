# Enhanced CURSED Collections Core Module Tests
# Comprehensive test suite for memory management, concurrent access, edge cases

yeet "testz"
yeet "collections_core"
yeet "atomic_drip"

# Test dynamic array operations with memory management
slay test_dynamic_array() {
    test_start("dynamic_array")
    
    sus arr *DynamicArray = dynamic_array_new(4)  # Initial capacity of 4
    assert_true(arr != cringe)
    assert_eq_int(dynamic_array_length(arr), 0)
    assert_eq_int(dynamic_array_capacity(arr), 4)
    
    # Add elements to trigger growth
    bestie i := 0; i < 10; i++ {
        dynamic_array_append(arr, i)
    }
    
    assert_eq_int(dynamic_array_length(arr), 10)
    assert_true(dynamic_array_capacity(arr) >= 10)
    
    # Test element access
    bestie i := 0; i < 10; i++ {
        sus value normie = dynamic_array_get(arr, i)
        assert_eq_int(value, i)
    }
    
    # Test element modification
    dynamic_array_set(arr, 5, 99)
    sus modified normie = dynamic_array_get(arr, 5)
    assert_eq_int(modified, 99)
    
    # Test removal
    sus removed normie = dynamic_array_remove_at(arr, 5)
    assert_eq_int(removed, 99)
    assert_eq_int(dynamic_array_length(arr), 9)
    
    dynamic_array_free(arr)
    
    vibez.spill("✅ Dynamic array test passed")
}

# Test hashmap operations with collision handling
slay test_hashmap_advanced() {
    test_start("hashmap_advanced")
    
    sus map *HashMap = hashmap_new(8)  # Small initial size to force collisions
    assert_true(map != cringe)
    assert_eq_int(hashmap_size(map), 0)
    
    # Insert many key-value pairs to test collision handling
    bestie i := 0; i < 100; i++ {
        sus key tea = "key" + i.tea()
        hashmap_insert(map, key, i * 2)
    }
    
    assert_eq_int(hashmap_size(map), 100)
    
    # Verify all values can be retrieved
    bestie i := 0; i < 100; i++ {
        sus key tea = "key" + i.tea()
        assert_true(hashmap_contains(map, key))
        sus value normie = hashmap_get(map, key)
        assert_eq_int(value, i * 2)
    }
    
    # Test update operation
    hashmap_insert(map, "key50", 9999)
    sus updated normie = hashmap_get(map, "key50")
    assert_eq_int(updated, 9999)
    assert_eq_int(hashmap_size(map), 100)  # Size shouldn't change
    
    # Test removal
    sus removed lit = hashmap_remove(map, "key50")
    assert_true(removed)
    assert_false(hashmap_contains(map, "key50"))
    assert_eq_int(hashmap_size(map), 99)
    
    # Test rehashing under load
    sus load_factor meal = hashmap_load_factor(map)
    assert_true(load_factor > 0.0)
    
    hashmap_free(map)
    
    vibez.spill("✅ Advanced hashmap test passed")
}

# Test linked list edge cases
slay test_linked_list_edge_cases() {
    test_start("linked_list_edge_cases")
    
    sus list *LinkedList = linked_list_new()
    assert_true(list != cringe)
    assert_eq_int(linked_list_length(list), 0)
    assert_true(linked_list_is_empty(list))
    
    # Test operations on empty list
    assert_eq_int(linked_list_get_first(list), 0)
    assert_eq_int(linked_list_get_last(list), 0)
    assert_false(linked_list_remove_first(list))
    assert_false(linked_list_remove_last(list))
    
    # Add single element
    linked_list_add_first(list, 42)
    assert_eq_int(linked_list_length(list), 1)
    assert_eq_int(linked_list_get_first(list), 42)
    assert_eq_int(linked_list_get_last(list), 42)
    
    # Remove single element
    sus removed lit = linked_list_remove_first(list)
    assert_true(removed)
    assert_eq_int(linked_list_length(list), 0)
    assert_true(linked_list_is_empty(list))
    
    # Test alternating add/remove pattern
    bestie i := 0; i < 10; i++ {
        linked_list_add_last(list, i)
        yo i % 2 == 0 {
            linked_list_remove_first(list)
        }
    }
    
    # Verify final state
    assert_eq_int(linked_list_length(list), 5)
    
    linked_list_free(list)
    
    vibez.spill("✅ Linked list edge cases test passed")
}

# Test binary tree operations
slay test_binary_tree() {
    test_start("binary_tree")
    
    sus tree *BinaryTree = binary_tree_new()
    assert_true(tree != cringe)
    assert_eq_int(binary_tree_size(tree), 0)
    assert_true(binary_tree_is_empty(tree))
    
    # Insert values in random order
    sus values []normie = []normie{50, 30, 70, 20, 40, 60, 80, 10, 25, 35, 45}
    bestie _, value := range values {
        binary_tree_insert(tree, value)
    }
    
    assert_eq_int(binary_tree_size(tree), 11)
    assert_false(binary_tree_is_empty(tree))
    
    # Test search operations
    bestie _, value := range values {
        assert_true(binary_tree_contains(tree, value))
    }
    
    assert_false(binary_tree_contains(tree, 99))
    assert_false(binary_tree_contains(tree, 0))
    
    # Test tree traversal
    sus inorder []normie = binary_tree_inorder_traversal(tree)
    assert_eq_int(len(inorder), 11)
    
    # Verify inorder traversal is sorted
    bestie i := 0; i < len(inorder) - 1; i++ {
        assert_true(inorder[i] < inorder[i+1])
    }
    
    # Test min/max values
    sus min_val normie = binary_tree_find_min(tree)
    sus max_val normie = binary_tree_find_max(tree)
    assert_eq_int(min_val, 10)
    assert_eq_int(max_val, 80)
    
    # Test deletion
    binary_tree_delete(tree, 50)  # Delete root
    assert_false(binary_tree_contains(tree, 50))
    assert_eq_int(binary_tree_size(tree), 10)
    
    binary_tree_free(tree)
    
    vibez.spill("✅ Binary tree test passed")
}

# Test concurrent access to collections
slay test_concurrent_access() {
    test_start("concurrent_access")
    
    sus shared_map *ConcurrentHashMap = concurrent_hashmap_new(16)
    sus counter *AtomicI32 = atomic_i32_new(0)
    
    # Simulate concurrent readers and writers
    sus num_threads normie = 5
    sus operations_per_thread normie = 100
    
    # Writer threads
    bestie thread_id := 0; thread_id < num_threads; thread_id++ {
        bestie op := 0; op < operations_per_thread; op++ {
            sus key tea = "thread" + thread_id.tea() + "_key" + op.tea()
            sus value normie = thread_id * 1000 + op
            concurrent_hashmap_insert(shared_map, key, value)
            atomic_increment_i32(counter)
        }
    }
    
    # Verify all insertions completed
    sus total_ops normie = atomic_load_i32(counter)
    assert_eq_int(total_ops, num_threads * operations_per_thread)
    assert_eq_int(concurrent_hashmap_size(shared_map), total_ops)
    
    # Reader threads
    atomic_store_i32(counter, 0)
    bestie thread_id := 0; thread_id < num_threads; thread_id++ {
        bestie op := 0; op < operations_per_thread; op++ {
            sus key tea = "thread" + thread_id.tea() + "_key" + op.tea()
            sus expected normie = thread_id * 1000 + op
            
            yo concurrent_hashmap_contains(shared_map, key) {
                sus actual normie = concurrent_hashmap_get(shared_map, key)
                assert_eq_int(actual, expected)
                atomic_increment_i32(counter)
            }
        }
    }
    
    sus successful_reads normie = atomic_load_i32(counter)
    assert_eq_int(successful_reads, total_ops)
    
    concurrent_hashmap_free(shared_map)
    
    vibez.spill("✅ Concurrent access test passed")
}

# Test memory pool allocator
slay test_memory_pool() {
    test_start("memory_pool")
    
    sus pool *MemoryPool = memory_pool_new(64, 100)  # 100 blocks of 64 bytes each
    assert_true(pool != cringe)
    assert_eq_int(memory_pool_block_size(pool), 64)
    assert_eq_int(memory_pool_total_blocks(pool), 100)
    assert_eq_int(memory_pool_available_blocks(pool), 100)
    
    # Allocate multiple blocks
    sus ptrs []*void = make([]*void, 50)
    bestie i := 0; i < 50; i++ {
        ptrs[i] = memory_pool_alloc(pool)
        assert_true(ptrs[i] != cringe)
    }
    
    assert_eq_int(memory_pool_available_blocks(pool), 50)
    
    # Write and read data to verify integrity
    bestie i := 0; i < 50; i++ {
        sus byte_ptr *byte = ptrs[i].(*byte)
        *byte_ptr = i.(byte)
    }
    
    bestie i := 0; i < 50; i++ {
        sus byte_ptr *byte = ptrs[i].(*byte)
        assert_eq_int(*byte_ptr.(normie), i)
    }
    
    # Free blocks
    bestie i := 0; i < 50; i++ {
        memory_pool_free(pool, ptrs[i])
    }
    
    assert_eq_int(memory_pool_available_blocks(pool), 100)
    
    memory_pool_destroy(pool)
    
    vibez.spill("✅ Memory pool test passed")
}

# Test circular buffer
slay test_circular_buffer() {
    test_start("circular_buffer")
    
    sus buffer *CircularBuffer = circular_buffer_new(8)
    assert_true(buffer != cringe)
    assert_eq_int(circular_buffer_capacity(buffer), 8)
    assert_eq_int(circular_buffer_length(buffer), 0)
    assert_true(circular_buffer_is_empty(buffer))
    assert_false(circular_buffer_is_full(buffer))
    
    # Fill buffer to capacity
    bestie i := 0; i < 8; i++ {
        assert_true(circular_buffer_enqueue(buffer, i))
    }
    
    assert_true(circular_buffer_is_full(buffer))
    assert_false(circular_buffer_is_empty(buffer))
    assert_eq_int(circular_buffer_length(buffer), 8)
    
    # Try to add one more (should fail)
    assert_false(circular_buffer_enqueue(buffer, 99))
    
    # Dequeue half the elements
    bestie i := 0; i < 4; i++ {
        sus value normie = circular_buffer_dequeue(buffer)
        assert_eq_int(value, i)
    }
    
    assert_eq_int(circular_buffer_length(buffer), 4)
    assert_false(circular_buffer_is_full(buffer))
    
    # Add more elements to test wraparound
    bestie i := 8; i < 12; i++ {
        assert_true(circular_buffer_enqueue(buffer, i))
    }
    
    assert_true(circular_buffer_is_full(buffer))
    
    # Verify correct order after wraparound
    sus expected []normie = []normie{4, 5, 6, 7, 8, 9, 10, 11}
    bestie i := 0; i < 8; i++ {
        sus value normie = circular_buffer_dequeue(buffer)
        assert_eq_int(value, expected[i])
    }
    
    assert_true(circular_buffer_is_empty(buffer))
    
    circular_buffer_free(buffer)
    
    vibez.spill("✅ Circular buffer test passed")
}

# Test priority queue operations
slay test_priority_queue() {
    test_start("priority_queue")
    
    sus pq *PriorityQueue = priority_queue_new(10)
    assert_true(pq != cringe)
    assert_eq_int(priority_queue_size(pq), 0)
    assert_true(priority_queue_is_empty(pq))
    
    # Insert elements with different priorities
    sus items []PriorityItem = []PriorityItem{
        {value: 10, priority: 1},
        {value: 30, priority: 3},
        {value: 20, priority: 2},
        {value: 50, priority: 5},
        {value: 40, priority: 4}
    }
    
    bestie _, item := range items {
        priority_queue_insert(pq, item.value, item.priority)
    }
    
    assert_eq_int(priority_queue_size(pq), 5)
    assert_false(priority_queue_is_empty(pq))
    
    # Extract elements (should come out in priority order)
    sus expected []normie = []normie{50, 40, 30, 20, 10}  # Highest priority first
    bestie i := 0; i < 5; i++ {
        sus value normie = priority_queue_extract_max(pq)
        assert_eq_int(value, expected[i])
    }
    
    assert_true(priority_queue_is_empty(pq))
    
    priority_queue_free(pq)
    
    vibez.spill("✅ Priority queue test passed")
}

# Test memory leak detection
slay test_memory_leak_detection() {
    test_start("memory_leak_detection")
    
    sus initial_usage normie = get_memory_usage()
    
    # Allocate and properly free memory
    bestie i := 0; i < 100; i++ {
        sus ptr *void = malloc(1024)
        assert_true(ptr != cringe)
        free(ptr)
    }
    
    sus after_proper_cleanup normie = get_memory_usage()
    sus diff normie = after_proper_cleanup - initial_usage
    assert_true(diff < 1024)  # Should be minimal difference
    
    # Test collections cleanup
    bestie i := 0; i < 10; i++ {
        sus list *LinkedList = linked_list_new()
        bestie j := 0; j < 100; j++ {
            linked_list_add_last(list, j)
        }
        linked_list_free(list)
    }
    
    sus final_usage normie = get_memory_usage()
    sus final_diff normie = final_usage - initial_usage
    assert_true(final_diff < 10240)  # Should not leak significantly
    
    vibez.spill("✅ Memory leak detection test passed")
}

# Test collection iteration and modification
slay test_concurrent_modification() {
    test_start("concurrent_modification")
    
    sus list *SafeLinkedList = safe_linked_list_new()
    
    # Add initial elements
    bestie i := 0; i < 10; i++ {
        safe_linked_list_add(list, i)
    }
    
    # Test safe iteration with concurrent modification
    sus iterator *SafeIterator = safe_linked_list_iterator(list)
    sus count normie = 0
    
    bestie safe_iterator_has_next(iterator) {
        sus value normie = safe_iterator_next(iterator)
        
        # Modify list during iteration (should be safe)
        yo count % 3 == 0 {
            safe_linked_list_add(list, value + 100)
        }
        
        count++
        
        # Prevent infinite loop
        yo count > 20 {
            break
        }
    }
    
    assert_true(count >= 10)  # At least original elements
    assert_true(safe_linked_list_size(list) > 10)  # Some elements added
    
    safe_iterator_free(iterator)
    safe_linked_list_free(list)
    
    vibez.spill("✅ Concurrent modification test passed")
}

# Stress test for all collections
slay test_collections_stress() {
    test_start("collections_stress")
    
    sus operations normie = 1000
    
    # Stress test dynamic array
    sus arr *DynamicArray = dynamic_array_new(1)
    bestie i := 0; i < operations; i++ {
        dynamic_array_append(arr, i)
        yo i % 100 == 99 {
            dynamic_array_remove_at(arr, i / 2)
        }
    }
    assert_true(dynamic_array_length(arr) > operations / 2)
    dynamic_array_free(arr)
    
    # Stress test hashmap
    sus map *HashMap = hashmap_new(1)
    bestie i := 0; i < operations; i++ {
        sus key tea = "stress_key_" + i.tea()
        hashmap_insert(map, key, i)
        yo i % 100 == 99 {
            sus old_key tea = "stress_key_" + (i - 50).tea()
            hashmap_remove(map, old_key)
        }
    }
    assert_true(hashmap_size(map) > operations / 2)
    hashmap_free(map)
    
    vibez.spill("✅ Collections stress test passed")
}

# Main test runner
slay main() {
    vibez.spill("🧪 Running Enhanced CURSED Collections Core Module Tests")
    vibez.spill("==============================================================")
    
    test_dynamic_array()
    test_hashmap_advanced()
    test_linked_list_edge_cases()
    test_binary_tree()
    test_concurrent_access()
    test_memory_pool()
    test_circular_buffer()
    test_priority_queue()
    test_memory_leak_detection()
    test_concurrent_modification()
    test_collections_stress()
    
    vibez.spill("==============================================================")
    print_test_summary()
    vibez.spill("🎉 All enhanced collections core tests completed!")
}
