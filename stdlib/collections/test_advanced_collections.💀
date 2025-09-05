// Test Suite for Advanced Collections Module

yeet "testz"
yeet "advanced_collections"

// ================================
// B-Tree Tests
// ================================

slay test_btree_basic_operations() {
    test_start("B-Tree Basic Operations")
    
    sus tree BTree = btree_new(3)
    
    // Test insertion
    tree = btree_insert(tree, "apple", "fruit")
    tree = btree_insert(tree, "banana", "yellow")
    tree = btree_insert(tree, "cherry", "red")
    tree = btree_insert(tree, "date", "sweet")
    
    // Test search
    assert_eq_string(btree_search(tree, "apple"), "fruit")
    assert_eq_string(btree_search(tree, "banana"), "yellow")
    assert_eq_string(btree_search(tree, "cherry"), "red")
    assert_eq_string(btree_search(tree, "date"), "sweet")
    
    // Test non-existent key
    assert_eq_string(btree_search(tree, "elephant"), "")
    
    vibez.spill("B-Tree basic operations: PASSED")
}

slay test_btree_large_dataset() {
    test_start("B-Tree Large Dataset")
    
    sus tree BTree = btree_new(5)
    
    // Insert many items to test splitting
    sus i normie = 0
    bestie i < 20 {
        sus key tea = "key" + tea(i)
        sus value tea = "value" + tea(i)
        tree = btree_insert(tree, key, value)
        i = i + 1
    }
    
    // Verify all items can be found
    i = 0
    bestie i < 20 {
        sus key tea = "key" + tea(i)
        sus expected_value tea = "value" + tea(i)
        sus actual_value tea = btree_search(tree, key)
        assert_eq_string(actual_value, expected_value)
        i = i + 1
    }
    
    vibez.spill("B-Tree large dataset: PASSED")
}

// ================================
// AVL Tree Tests
// ================================

slay test_avl_basic_operations() {
    test_start("AVL Tree Basic Operations")
    
    sus tree AVLTree = avl_new()
    
    // Test insertion
    tree = avl_insert(tree, "delta", "d")
    tree = avl_insert(tree, "beta", "b")
    tree = avl_insert(tree, "alpha", "a")
    tree = avl_insert(tree, "gamma", "g")
    
    // Test search
    assert_eq_string(avl_search(tree, "alpha"), "a")
    assert_eq_string(avl_search(tree, "beta"), "b")
    assert_eq_string(avl_search(tree, "gamma"), "g")
    assert_eq_string(avl_search(tree, "delta"), "d")
    
    // Test non-existent key
    assert_eq_string(avl_search(tree, "epsilon"), "")
    
    vibez.spill("AVL Tree basic operations: PASSED")
}

slay test_avl_balancing() {
    test_start("AVL Tree Balancing")
    
    sus tree AVLTree = avl_new()
    
    // Insert in ascending order to trigger rotations
    tree = avl_insert(tree, "1", "one")
    tree = avl_insert(tree, "2", "two")
    tree = avl_insert(tree, "3", "three")
    tree = avl_insert(tree, "4", "four")
    tree = avl_insert(tree, "5", "five")
    
    // Verify all items are still accessible
    assert_eq_string(avl_search(tree, "1"), "one")
    assert_eq_string(avl_search(tree, "2"), "two")
    assert_eq_string(avl_search(tree, "3"), "three")
    assert_eq_string(avl_search(tree, "4"), "four")
    assert_eq_string(avl_search(tree, "5"), "five")
    
    vibez.spill("AVL Tree balancing: PASSED")
}

// ================================
// Priority Queue Tests
// ================================

slay test_priority_queue_basic() {
    test_start("Priority Queue Basic Operations")
    
    sus pq PriorityQueue = priority_queue_new()
    
    // Test empty queue
    assert_true(priority_queue_is_empty(pq))
    assert_eq_int(priority_queue_size(pq), 0)
    
    // Test insertion
    pq = priority_queue_insert(pq, "low", 1)
    pq = priority_queue_insert(pq, "high", 10)
    pq = priority_queue_insert(pq, "medium", 5)
    
    assert_false(priority_queue_is_empty(pq))
    assert_eq_int(priority_queue_size(pq), 3)
    
    // Test peek (should be highest priority)
    assert_eq_string(priority_queue_peek(pq), "high")
    
    vibez.spill("Priority Queue basic operations: PASSED")
}

slay test_priority_queue_extraction() {
    test_start("Priority Queue Extraction")
    
    sus pq PriorityQueue = priority_queue_new()
    
    // Insert items with different priorities
    pq = priority_queue_insert(pq, "task1", 3)
    pq = priority_queue_insert(pq, "task2", 1)
    pq = priority_queue_insert(pq, "task3", 7)
    pq = priority_queue_insert(pq, "task4", 5)
    
    // Extract in priority order
    assert_eq_string(priority_queue_extract_max(pq), "task3")
    assert_eq_string(priority_queue_extract_max(pq), "task4")
    assert_eq_string(priority_queue_extract_max(pq), "task1")
    assert_eq_string(priority_queue_extract_max(pq), "task2")
    
    // Queue should be empty now
    assert_true(priority_queue_is_empty(pq))
    
    vibez.spill("Priority Queue extraction: PASSED")
}

// ================================
// Concurrent HashMap Tests
// ================================

slay test_concurrent_hashmap_basic() {
    test_start("Concurrent HashMap Basic Operations")
    
    sus chm ConcurrentHashMap = concurrent_hashmap_new(4)
    
    // Test insertion
    chm = concurrent_hashmap_insert(chm, "key1", "value1")
    chm = concurrent_hashmap_insert(chm, "key2", "value2")
    chm = concurrent_hashmap_insert(chm, "key3", "value3")
    
    // Test retrieval
    assert_eq_string(concurrent_hashmap_get(chm, "key1"), "value1")
    assert_eq_string(concurrent_hashmap_get(chm, "key2"), "value2")
    assert_eq_string(concurrent_hashmap_get(chm, "key3"), "value3")
    
    // Test contains key
    assert_true(concurrent_hashmap_contains_key(chm, "key1"))
    assert_true(concurrent_hashmap_contains_key(chm, "key2"))
    assert_false(concurrent_hashmap_contains_key(chm, "nonexistent"))
    
    vibez.spill("Concurrent HashMap basic operations: PASSED")
}

slay test_concurrent_hashmap_segments() {
    test_start("Concurrent HashMap Segmentation")
    
    sus chm ConcurrentHashMap = concurrent_hashmap_new(8)
    
    // Insert many items across segments
    sus i normie = 0
    bestie i < 24 {
        sus key tea = "segment_key_" + tea(i)
        sus value tea = "segment_value_" + tea(i)
        chm = concurrent_hashmap_insert(chm, key, value)
        i = i + 1
    }
    
    // Verify all items can be retrieved
    i = 0
    bestie i < 24 {
        sus key tea = "segment_key_" + tea(i)
        sus expected_value tea = "segment_value_" + tea(i)
        sus actual_value tea = concurrent_hashmap_get(chm, key)
        assert_eq_string(actual_value, expected_value)
        i = i + 1
    }
    
    assert_eq_int(concurrent_hashmap_size(chm), 24)
    
    vibez.spill("Concurrent HashMap segmentation: PASSED")
}

// ================================
// Performance Tests
// ================================

slay test_performance_comparison() {
    test_start("Performance Comparison")
    
    vibez.spill("Performance Test: B-Tree vs AVL Tree")
    
    // B-Tree performance
    sus btree BTree = btree_new(5)
    sus i normie = 0
    bestie i < 100 {
        sus key tea = "perf_key_" + tea(i)
        sus value tea = "perf_value_" + tea(i)
        btree = btree_insert(btree, key, value)
        i = i + 1
    }
    
    // AVL Tree performance
    sus avl AVLTree = avl_new()
    i = 0
    bestie i < 100 {
        sus key tea = "perf_key_" + tea(i)
        sus value tea = "perf_value_" + tea(i)
        avl = avl_insert(avl, key, value)
        i = i + 1
    }
    
    // Concurrent HashMap performance
    sus chm ConcurrentHashMap = concurrent_hashmap_new(16)
    i = 0
    bestie i < 100 {
        sus key tea = "perf_key_" + tea(i)
        sus value tea = "perf_value_" + tea(i)
        chm = concurrent_hashmap_insert(chm, key, value)
        i = i + 1
    }
    
    vibez.spill("Performance test completed successfully")
}

// ================================
// Memory Efficiency Tests
// ================================

slay test_memory_efficiency() {
    test_start("Memory Efficiency")
    
    vibez.spill("Testing memory efficiency of advanced data structures")
    
    // Test memory usage reporting
    memory_usage_report()
    
    // Create large data structures
    sus btree BTree = btree_new(10)
    sus avl AVLTree = avl_new()
    sus pq PriorityQueue = priority_queue_new()
    
    // Insert many items
    sus i normie = 0
    bestie i < 50 {
        sus key tea = "memory_test_" + tea(i)
        sus value tea = "memory_value_" + tea(i)
        
        btree = btree_insert(btree, key, value)
        avl = avl_insert(avl, key, value)
        pq = priority_queue_insert(pq, key, i)
        
        i = i + 1
    }
    
    // Trigger garbage collection
    trigger_gc()
    
    // Check memory pool info
    memory_pool_info()
    
    vibez.spill("Memory efficiency test completed")
}

// ================================
// Edge Case Tests
// ================================

slay test_edge_cases() {
    test_start("Edge Cases")
    
    // Test empty structures
    sus empty_btree BTree = btree_new(3)
    assert_eq_string(btree_search(empty_btree, "key"), "")
    
    sus empty_avl AVLTree = avl_new()
    assert_eq_string(avl_search(empty_avl, "key"), "")
    
    sus empty_pq PriorityQueue = priority_queue_new()
    assert_eq_string(priority_queue_peek(empty_pq), "")
    assert_eq_string(priority_queue_extract_max(empty_pq), "")
    
    // Test single element structures
    sus single_btree BTree = btree_new(3)
    single_btree = btree_insert(single_btree, "single", "value")
    assert_eq_string(btree_search(single_btree, "single"), "value")
    
    sus single_avl AVLTree = avl_new()
    single_avl = avl_insert(single_avl, "single", "value")
    assert_eq_string(avl_search(single_avl, "single"), "value")
    
    sus single_pq PriorityQueue = priority_queue_new()
    single_pq = priority_queue_insert(single_pq, "single", 1)
    assert_eq_string(priority_queue_peek(single_pq), "single")
    assert_eq_string(priority_queue_extract_max(single_pq), "single")
    assert_true(priority_queue_is_empty(single_pq))
    
    vibez.spill("Edge cases: PASSED")
}

// ================================
// Integration Tests
// ================================

slay test_integration_scenarios() {
    test_start("Integration Scenarios")
    
    // Scenario 1: Search engine indexing
    vibez.spill("Scenario 1: Search Engine Indexing")
    sus search_index BTree = btree_new(10)
    search_index = btree_insert(search_index, "computer", "technology")
    search_index = btree_insert(search_index, "science", "research")
    search_index = btree_insert(search_index, "programming", "coding")
    
    assert_eq_string(btree_search(search_index, "computer"), "technology")
    assert_eq_string(btree_search(search_index, "science"), "research")
    
    // Scenario 2: Task scheduling
    vibez.spill("Scenario 2: Task Scheduling")
    sus task_queue PriorityQueue = priority_queue_new()
    task_queue = priority_queue_insert(task_queue, "urgent_task", 10)
    task_queue = priority_queue_insert(task_queue, "normal_task", 5)
    task_queue = priority_queue_insert(task_queue, "low_priority", 1)
    
    assert_eq_string(priority_queue_extract_max(task_queue), "urgent_task")
    assert_eq_string(priority_queue_extract_max(task_queue), "normal_task")
    assert_eq_string(priority_queue_extract_max(task_queue), "low_priority")
    
    // Scenario 3: Distributed caching
    vibez.spill("Scenario 3: Distributed Caching")
    sus cache ConcurrentHashMap = concurrent_hashmap_new(8)
    cache = concurrent_hashmap_insert(cache, "user:123", "user_data")
    cache = concurrent_hashmap_insert(cache, "session:456", "session_data")
    cache = concurrent_hashmap_insert(cache, "product:789", "product_data")
    
    assert_eq_string(concurrent_hashmap_get(cache, "user:123"), "user_data")
    assert_eq_string(concurrent_hashmap_get(cache, "session:456"), "session_data")
    assert_eq_string(concurrent_hashmap_get(cache, "product:789"), "product_data")
    
    vibez.spill("Integration scenarios: PASSED")
}

// ================================
// Main Test Runner
// ================================

slay run_all_advanced_collections_tests() {
    vibez.spill("🚀 Running Advanced Collections Test Suite")
    vibez.spill("==========================================")
    
    // B-Tree tests
    test_btree_basic_operations()
    test_btree_large_dataset()
    
    // AVL Tree tests
    test_avl_basic_operations()
    test_avl_balancing()
    
    // Priority Queue tests
    test_priority_queue_basic()
    test_priority_queue_extraction()
    
    // Concurrent HashMap tests
    test_concurrent_hashmap_basic()
    test_concurrent_hashmap_segments()
    
    // Performance tests
    test_performance_comparison()
    
    // Memory efficiency tests
    test_memory_efficiency()
    
    // Edge case tests
    test_edge_cases()
    
    // Integration tests
    test_integration_scenarios()
    
    print_test_summary()
    
    vibez.spill("🎉 Advanced Collections Test Suite Complete!")
}

// Auto-run tests when this file is executed
run_all_advanced_collections_tests()
