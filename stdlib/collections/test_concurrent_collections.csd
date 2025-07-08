// Test Suite for Concurrent Collections Module

yeet "testz"
yeet "concurrent_collections"

// ================================
// Lock-Free Stack Tests
// ================================

slay test_lockfree_stack_basic() {
    test_start("Lock-Free Stack Basic Operations")
    
    sus stack LockFreeStack = lockfree_stack_new()
    
    // Test empty stack
    assert_true(lockfree_stack_is_empty(stack))
    assert_eq_int(lockfree_stack_size(stack), 0)
    assert_eq_string(lockfree_stack_pop(stack), "")
    
    // Test push operations
    lockfree_stack_push(stack, "first")
    lockfree_stack_push(stack, "second")
    lockfree_stack_push(stack, "third")
    
    assert_false(lockfree_stack_is_empty(stack))
    assert_eq_int(lockfree_stack_size(stack), 3)
    
    // Test pop operations (LIFO order)
    assert_eq_string(lockfree_stack_pop(stack), "third")
    assert_eq_string(lockfree_stack_pop(stack), "second")
    assert_eq_string(lockfree_stack_pop(stack), "first")
    
    assert_true(lockfree_stack_is_empty(stack))
    assert_eq_int(lockfree_stack_size(stack), 0)
    
    vibez.spill("Lock-Free Stack basic operations: PASSED")
}

slay test_lockfree_stack_concurrent() {
    test_start("Lock-Free Stack Concurrent Operations")
    
    sus stack LockFreeStack = lockfree_stack_new()
    
    // Simulate concurrent pushes
    lockfree_stack_push(stack, "item1")
    lockfree_stack_push(stack, "item2")
    lockfree_stack_push(stack, "item3")
    lockfree_stack_push(stack, "item4")
    lockfree_stack_push(stack, "item5")
    
    assert_eq_int(lockfree_stack_size(stack), 5)
    
    // Simulate concurrent pops
    sus item1 tea = lockfree_stack_pop(stack)
    sus item2 tea = lockfree_stack_pop(stack)
    sus item3 tea = lockfree_stack_pop(stack)
    
    assert_eq_int(lockfree_stack_size(stack), 2)
    
    // Verify items are not empty
    assert_true(item1 != "")
    assert_true(item2 != "")
    assert_true(item3 != "")
    
    vibez.spill("Lock-Free Stack concurrent operations: PASSED")
}

// ================================
// Lock-Free Queue Tests
// ================================

slay test_lockfree_queue_basic() {
    test_start("Lock-Free Queue Basic Operations")
    
    sus queue LockFreeQueue = lockfree_queue_new()
    
    // Test empty queue
    assert_true(lockfree_queue_is_empty(queue))
    assert_eq_int(lockfree_queue_size(queue), 0)
    assert_eq_string(lockfree_queue_dequeue(queue), "")
    
    // Test enqueue operations
    lockfree_queue_enqueue(queue, "first")
    lockfree_queue_enqueue(queue, "second")
    lockfree_queue_enqueue(queue, "third")
    
    assert_false(lockfree_queue_is_empty(queue))
    assert_eq_int(lockfree_queue_size(queue), 3)
    
    // Test dequeue operations (FIFO order)
    assert_eq_string(lockfree_queue_dequeue(queue), "first")
    assert_eq_string(lockfree_queue_dequeue(queue), "second")
    assert_eq_string(lockfree_queue_dequeue(queue), "third")
    
    assert_true(lockfree_queue_is_empty(queue))
    assert_eq_int(lockfree_queue_size(queue), 0)
    
    vibez.spill("Lock-Free Queue basic operations: PASSED")
}

slay test_lockfree_queue_concurrent() {
    test_start("Lock-Free Queue Concurrent Operations")
    
    sus queue LockFreeQueue = lockfree_queue_new()
    
    // Simulate concurrent enqueues
    lockfree_queue_enqueue(queue, "msg1")
    lockfree_queue_enqueue(queue, "msg2")
    lockfree_queue_enqueue(queue, "msg3")
    lockfree_queue_enqueue(queue, "msg4")
    
    assert_eq_int(lockfree_queue_size(queue), 4)
    
    // Simulate mixed enqueue/dequeue operations
    sus msg1 tea = lockfree_queue_dequeue(queue)
    lockfree_queue_enqueue(queue, "msg5")
    sus msg2 tea = lockfree_queue_dequeue(queue)
    
    assert_eq_string(msg1, "msg1")
    assert_eq_string(msg2, "msg2")
    assert_eq_int(lockfree_queue_size(queue), 3)
    
    vibez.spill("Lock-Free Queue concurrent operations: PASSED")
}

// ================================
// Concurrent HashMap Advanced Tests
// ================================

slay test_concurrent_hashmap_advanced_basic() {
    test_start("Concurrent HashMap Advanced Basic Operations")
    
    sus chm ConcurrentHashMapAdvanced = concurrent_hashmap_advanced_new(8)
    
    // Test insertion
    concurrent_hashmap_advanced_insert(chm, "key1", "value1")
    concurrent_hashmap_advanced_insert(chm, "key2", "value2")
    concurrent_hashmap_advanced_insert(chm, "key3", "value3")
    
    assert_eq_int(concurrent_hashmap_advanced_size(chm), 3)
    
    // Test retrieval
    assert_eq_string(concurrent_hashmap_advanced_get(chm, "key1"), "value1")
    assert_eq_string(concurrent_hashmap_advanced_get(chm, "key2"), "value2")
    assert_eq_string(concurrent_hashmap_advanced_get(chm, "key3"), "value3")
    
    // Test non-existent key
    assert_eq_string(concurrent_hashmap_advanced_get(chm, "nonexistent"), "")
    
    vibez.spill("Concurrent HashMap Advanced basic operations: PASSED")
}

slay test_concurrent_hashmap_advanced_concurrent() {
    test_start("Concurrent HashMap Advanced Concurrent Operations")
    
    sus chm ConcurrentHashMapAdvanced = concurrent_hashmap_advanced_new(16)
    
    // Simulate concurrent insertions across different buckets
    sus i normie = 0
    bestie i < 32 {
        sus key tea = "concurrent_key_" + tea(i)
        sus value tea = "concurrent_value_" + tea(i)
        concurrent_hashmap_advanced_insert(chm, key, value)
        i = i + 1
    }
    
    assert_eq_int(concurrent_hashmap_advanced_size(chm), 32)
    
    // Verify all items can be retrieved
    i = 0
    bestie i < 32 {
        sus key tea = "concurrent_key_" + tea(i)
        sus expected_value tea = "concurrent_value_" + tea(i)
        sus actual_value tea = concurrent_hashmap_advanced_get(chm, key)
        assert_eq_string(actual_value, expected_value)
        i = i + 1
    }
    
    // Test concurrent removals
    i = 0
    bestie i < 16 {
        sus key tea = "concurrent_key_" + tea(i)
        assert_true(concurrent_hashmap_advanced_remove(chm, key))
        i = i + 1
    }
    
    assert_eq_int(concurrent_hashmap_advanced_size(chm), 16)
    
    vibez.spill("Concurrent HashMap Advanced concurrent operations: PASSED")
}

// ================================
// Reader-Writer Lock Tests
// ================================

slay test_rwlock_basic() {
    test_start("Reader-Writer Lock Basic Operations")
    
    sus rwlock ReadWriteLock = rwlock_new()
    
    // Test read lock
    rwlock_read_lock(rwlock)
    rwlock_read_unlock(rwlock)
    
    // Test write lock
    rwlock_write_lock(rwlock)
    rwlock_write_unlock(rwlock)
    
    // Test multiple read locks
    rwlock_read_lock(rwlock)
    rwlock_read_lock(rwlock)
    rwlock_read_unlock(rwlock)
    rwlock_read_unlock(rwlock)
    
    vibez.spill("Reader-Writer Lock basic operations: PASSED")
}

// ================================
// Concurrent Set Tests
// ================================

slay test_concurrent_set_basic() {
    test_start("Concurrent Set Basic Operations")
    
    sus set ConcurrentSet = concurrent_set_new(8)
    
    // Test add operations
    concurrent_set_add(set, "apple")
    concurrent_set_add(set, "banana")
    concurrent_set_add(set, "cherry")
    concurrent_set_add(set, "apple")  // Duplicate
    
    assert_eq_int(concurrent_set_size(set), 3)  // Duplicates should not increase size
    
    // Test contains operations
    assert_true(concurrent_set_contains(set, "apple"))
    assert_true(concurrent_set_contains(set, "banana"))
    assert_true(concurrent_set_contains(set, "cherry"))
    assert_false(concurrent_set_contains(set, "orange"))
    
    // Test remove operations
    assert_true(concurrent_set_remove(set, "banana"))
    assert_false(concurrent_set_contains(set, "banana"))
    assert_eq_int(concurrent_set_size(set), 2)
    
    // Test remove non-existent item
    assert_false(concurrent_set_remove(set, "grape"))
    
    vibez.spill("Concurrent Set basic operations: PASSED")
}

// ================================
// Work-Stealing Queue Tests
// ================================

slay test_work_stealing_queue_basic() {
    test_start("Work-Stealing Queue Basic Operations")
    
    sus wsq WorkStealingQueue = work_stealing_queue_new(10)
    
    // Test empty queue
    assert_true(work_stealing_queue_is_empty(wsq))
    assert_eq_int(work_stealing_queue_size(wsq), 0)
    assert_eq_string(work_stealing_queue_pop(wsq), "")
    assert_eq_string(work_stealing_queue_steal(wsq), "")
    
    // Test push operations
    assert_true(work_stealing_queue_push(wsq, "task1"))
    assert_true(work_stealing_queue_push(wsq, "task2"))
    assert_true(work_stealing_queue_push(wsq, "task3"))
    
    assert_false(work_stealing_queue_is_empty(wsq))
    assert_eq_int(work_stealing_queue_size(wsq), 3)
    
    // Test pop (LIFO from owner)
    assert_eq_string(work_stealing_queue_pop(wsq), "task3")
    assert_eq_int(work_stealing_queue_size(wsq), 2)
    
    // Test steal (FIFO from thieves)
    assert_eq_string(work_stealing_queue_steal(wsq), "task1")
    assert_eq_int(work_stealing_queue_size(wsq), 1)
    
    // Remaining item
    assert_eq_string(work_stealing_queue_pop(wsq), "task2")
    assert_true(work_stealing_queue_is_empty(wsq))
    
    vibez.spill("Work-Stealing Queue basic operations: PASSED")
}

slay test_work_stealing_queue_capacity() {
    test_start("Work-Stealing Queue Capacity")
    
    sus wsq WorkStealingQueue = work_stealing_queue_new(3)
    
    // Fill to capacity
    assert_true(work_stealing_queue_push(wsq, "item1"))
    assert_true(work_stealing_queue_push(wsq, "item2"))
    assert_true(work_stealing_queue_push(wsq, "item3"))
    assert_eq_int(work_stealing_queue_size(wsq), 3)
    
    // Try to exceed capacity
    assert_false(work_stealing_queue_push(wsq, "item4"))
    assert_eq_int(work_stealing_queue_size(wsq), 3)
    
    // Make space and retry
    assert_eq_string(work_stealing_queue_pop(wsq), "item3")
    assert_true(work_stealing_queue_push(wsq, "item4"))
    assert_eq_int(work_stealing_queue_size(wsq), 3)
    
    vibez.spill("Work-Stealing Queue capacity: PASSED")
}

// ================================
// Concurrent Statistics Tests
// ================================

slay test_concurrent_stats() {
    test_start("Concurrent Statistics")
    
    sus stats ConcurrentStats = concurrent_stats_new()
    
    // Test increment operations
    concurrent_stats_increment_operations(stats)
    concurrent_stats_increment_successful(stats)
    concurrent_stats_increment_operations(stats)
    concurrent_stats_increment_failed(stats)
    concurrent_stats_increment_operations(stats)
    concurrent_stats_increment_successful(stats)
    
    // Report statistics
    concurrent_stats_report(stats)
    
    vibez.spill("Concurrent Statistics: PASSED")
}

// ================================
// Performance Tests
// ================================

slay test_concurrent_performance() {
    test_start("Concurrent Collections Performance")
    
    vibez.spill("Performance test: Lock-Free Stack")
    sus stack LockFreeStack = lockfree_stack_new()
    sus i normie = 0
    bestie i < 1000 {
        lockfree_stack_push(stack, "perf_item_" + tea(i))
        i = i + 1
    }
    
    i = 0
    bestie i < 1000 {
        sus item tea = lockfree_stack_pop(stack)
        i = i + 1
    }
    
    vibez.spill("Performance test: Lock-Free Queue")
    sus queue LockFreeQueue = lockfree_queue_new()
    i = 0
    bestie i < 1000 {
        lockfree_queue_enqueue(queue, "perf_msg_" + tea(i))
        i = i + 1
    }
    
    i = 0
    bestie i < 1000 {
        sus msg tea = lockfree_queue_dequeue(queue)
        i = i + 1
    }
    
    vibez.spill("Performance test: Concurrent HashMap")
    sus chm ConcurrentHashMapAdvanced = concurrent_hashmap_advanced_new(32)
    i = 0
    bestie i < 1000 {
        sus key tea = "perf_key_" + tea(i)
        sus value tea = "perf_value_" + tea(i)
        concurrent_hashmap_advanced_insert(chm, key, value)
        i = i + 1
    }
    
    i = 0
    bestie i < 1000 {
        sus key tea = "perf_key_" + tea(i)
        sus value tea = concurrent_hashmap_advanced_get(chm, key)
        i = i + 1
    }
    
    vibez.spill("Concurrent Collections Performance: PASSED")
}

// ================================
// Memory Consistency Tests
// ================================

slay test_memory_consistency() {
    test_start("Memory Consistency and Barriers")
    
    // Test memory barriers
    memory_barrier_full()
    memory_barrier_acquire()
    memory_barrier_release()
    
    // Test atomic operations consistency
    sus value normie = 42
    sus ptr *normie = &value
    
    sus original normie = atomic_load_int(ptr)
    atomic_store_int(ptr, 100)
    sus new_value normie = atomic_load_int(ptr)
    
    assert_eq_int(original, 42)
    assert_eq_int(new_value, 100)
    
    // Test compare-and-swap
    assert_true(atomic_compare_and_swap_int(ptr, 100, 200))
    assert_eq_int(atomic_load_int(ptr), 200)
    assert_false(atomic_compare_and_swap_int(ptr, 100, 300))
    assert_eq_int(atomic_load_int(ptr), 200)
    
    // Test fetch-and-add
    sus old_val normie = atomic_fetch_add_int(ptr, 50)
    assert_eq_int(old_val, 200)
    assert_eq_int(atomic_load_int(ptr), 250)
    
    vibez.spill("Memory Consistency and Barriers: PASSED")
}

// ================================
// Integration Tests
// ================================

slay test_concurrent_integration() {
    test_start("Concurrent Collections Integration")
    
    // Scenario: Producer-Consumer with Lock-Free Queue
    vibez.spill("Scenario 1: Producer-Consumer Pattern")
    sus producer_consumer_queue LockFreeQueue = lockfree_queue_new()
    
    // Producer produces items
    sus i normie = 0
    bestie i < 10 {
        sus item tea = "product_" + tea(i)
        lockfree_queue_enqueue(producer_consumer_queue, item)
        i = i + 1
    }
    
    // Consumer consumes items
    sus consumed_count normie = 0
    bestie !lockfree_queue_is_empty(producer_consumer_queue) {
        sus item tea = lockfree_queue_dequeue(producer_consumer_queue)
        lowkey item != "" {
            consumed_count = consumed_count + 1
        }
    }
    
    assert_eq_int(consumed_count, 10)
    
    // Scenario: Work Distribution with Work-Stealing Queues
    vibez.spill("Scenario 2: Work Distribution Pattern")
    sus work_queue WorkStealingQueue = work_stealing_queue_new(20)
    
    // Fill work queue
    i = 0
    bestie i < 15 {
        sus task tea = "work_task_" + tea(i)
        work_stealing_queue_push(work_queue, task)
        i = i + 1
    }
    
    // Simulate work stealing
    sus owner_tasks normie = 0
    sus thief_tasks normie = 0
    
    // Owner takes some tasks
    sus j normie = 0
    bestie j < 5 {
        sus task tea = work_stealing_queue_pop(work_queue)
        lowkey task != "" {
            owner_tasks = owner_tasks + 1
        }
        j = j + 1
    }
    
    // Thief steals some tasks
    j = 0
    bestie j < 5 {
        sus task tea = work_stealing_queue_steal(work_queue)
        lowkey task != "" {
            thief_tasks = thief_tasks + 1
        }
        j = j + 1
    }
    
    assert_eq_int(owner_tasks + thief_tasks, 10)
    assert_eq_int(work_stealing_queue_size(work_queue), 5)
    
    vibez.spill("Concurrent Collections Integration: PASSED")
}

// ================================
// Main Test Runner
// ================================

slay run_all_concurrent_collections_tests() {
    vibez.spill("🔄 Running Concurrent Collections Test Suite")
    vibez.spill("============================================")
    
    // Lock-Free Stack tests
    test_lockfree_stack_basic()
    test_lockfree_stack_concurrent()
    
    // Lock-Free Queue tests
    test_lockfree_queue_basic()
    test_lockfree_queue_concurrent()
    
    // Concurrent HashMap Advanced tests
    test_concurrent_hashmap_advanced_basic()
    test_concurrent_hashmap_advanced_concurrent()
    
    // Reader-Writer Lock tests
    test_rwlock_basic()
    
    // Concurrent Set tests
    test_concurrent_set_basic()
    
    // Work-Stealing Queue tests
    test_work_stealing_queue_basic()
    test_work_stealing_queue_capacity()
    
    // Concurrent Statistics tests
    test_concurrent_stats()
    
    // Performance tests
    test_concurrent_performance()
    
    // Memory consistency tests
    test_memory_consistency()
    
    // Integration tests
    test_concurrent_integration()
    
    print_test_summary()
    
    vibez.spill("🎉 Concurrent Collections Test Suite Complete!")
}

// Auto-run tests when this file is executed
run_all_concurrent_collections_tests()
