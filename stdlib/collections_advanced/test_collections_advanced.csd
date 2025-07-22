yeet "testz"
yeet "collections_advanced"

slay test_hashmap() {
    test_start("HashMap Operations")
    
    sus map := new_hashmap<normie, tea>(10) fr fr Test put and get
    hashmap_put(map, 1, "one")
    hashmap_put(map, 2, "two")
    hashmap_put(map, 3, "three")
    
    sus value1, exists1 := hashmap_get(map, 1)
    assert_true(exists1)
    assert_eq_string(value1, "one")
    
    sus value2, exists2 := hashmap_get(map, 2)
    assert_true(exists2)
    assert_eq_string(value2, "two") fr fr Test non-existent key
    sus value_none, exists_none := hashmap_get(map, 99)
    assert_false(exists_none) fr fr Test overwrite
    hashmap_put(map, 1, "ONE")
    sus value_updated, exists_updated := hashmap_get(map, 1)
    assert_true(exists_updated)
    assert_eq_string(value_updated, "ONE")
    
    vibez.spill("HashMap tests passed")
}

slay test_arraylist() {
    test_start("ArrayList Operations")
    
    sus list := new_arraylist<normie>(5) fr fr Test add and get
    arraylist_add(list, 10)
    arraylist_add(list, 20)
    arraylist_add(list, 30)
    arraylist_add(list, 40)
    arraylist_add(list, 50)
    
    sus val0, exists0 := arraylist_get(list, 0)
    assert_true(exists0)
    assert_eq_int(val0, 10)
    
    sus val2, exists2 := arraylist_get(list, 2)
    assert_true(exists2)
    assert_eq_int(val2, 30)
    
    sus val4, exists4 := arraylist_get(list, 4)
    assert_true(exists4)
    assert_eq_int(val4, 50) fr fr Test bounds checking
    sus val_out, exists_out := arraylist_get(list, 10)
    assert_false(exists_out) fr fr Test dynamic resizing
    arraylist_add(list, 60) fr fr Should trigger resize
    sus val5, exists5 := arraylist_get(list, 5)
    assert_true(exists5)
    assert_eq_int(val5, 60)
    
    vibez.spill("ArrayList tests passed")
}

slay test_linkedlist() {
    test_start("LinkedList Operations")
    
    sus list := new_linkedlist<normie>() fr fr Test add
    linkedlist_add(list, 100)
    linkedlist_add(list, 200)
    linkedlist_add(list, 300)
    
    assert_eq_int(list.size, 3) fr fr Test remove
    sus val1, success1 := linkedlist_remove_first(list)
    assert_true(success1)
    assert_eq_int(val1, 100)
    assert_eq_int(list.size, 2)
    
    sus val2, success2 := linkedlist_remove_first(list)
    assert_true(success2)
    assert_eq_int(val2, 200)
    assert_eq_int(list.size, 1)
    
    sus val3, success3 := linkedlist_remove_first(list)
    assert_true(success3)
    assert_eq_int(val3, 300)
    assert_eq_int(list.size, 0) fr fr Test empty list
    sus val_empty, success_empty := linkedlist_remove_first(list)
    assert_false(success_empty)
    
    vibez.spill("LinkedList tests passed")
}

slay test_stack() {
    test_start("Stack Operations")
    
    sus stack := new_stack<normie>() fr fr Test push and pop
    stack_push(stack, 1)
    stack_push(stack, 2)
    stack_push(stack, 3) fr fr Test peek
    sus peek_val, peek_success := stack_peek(stack)
    assert_true(peek_success)
    assert_eq_int(peek_val, 3) fr fr Test pop (LIFO order)
    sus val1, success1 := stack_pop(stack)
    assert_true(success1)
    assert_eq_int(val1, 3)
    
    sus val2, success2 := stack_pop(stack)
    assert_true(success2)
    assert_eq_int(val2, 2)
    
    sus val3, success3 := stack_pop(stack)
    assert_true(success3)
    assert_eq_int(val3, 1) fr fr Test empty stack
    sus val_empty, success_empty := stack_pop(stack)
    assert_false(success_empty)
    
    vibez.spill("Stack tests passed")
}

slay test_queue() {
    test_start("Queue Operations")
    
    sus queue := new_queue<normie>() fr fr Test enqueue and dequeue
    queue_enqueue(queue, 1)
    queue_enqueue(queue, 2)
    queue_enqueue(queue, 3)
    
    assert_eq_int(queue_size(queue), 3) fr fr Test dequeue (FIFO order)
    sus val1, success1 := queue_dequeue(queue)
    assert_true(success1)
    assert_eq_int(val1, 1)
    
    sus val2, success2 := queue_dequeue(queue)
    assert_true(success2)
    assert_eq_int(val2, 2)
    
    sus val3, success3 := queue_dequeue(queue)
    assert_true(success3)
    assert_eq_int(val3, 3)
    
    assert_eq_int(queue_size(queue), 0) fr fr Test empty queue
    sus val_empty, success_empty := queue_dequeue(queue)
    assert_false(success_empty)
    
    vibez.spill("Queue tests passed")
}

slay test_set() {
    test_start("Set Operations")
    
    sus set := new_set<normie>() fr fr Test add and contains
    set_add(set, 10)
    set_add(set, 20)
    set_add(set, 30)
    
    assert_true(set_contains(set, 10))
    assert_true(set_contains(set, 20))
    assert_true(set_contains(set, 30))
    assert_false(set_contains(set, 40)) fr fr Test duplicate add (should not affect membership)
    set_add(set, 10)
    assert_true(set_contains(set, 10))
    
    vibez.spill("Set tests passed")
}

slay test_bst() {
    test_start("Binary Search Tree Operations")
    
    sus tree := new_bst<normie>() fr fr Test insert and search
    bst_insert(tree, 50)
    bst_insert(tree, 30)
    bst_insert(tree, 70)
    bst_insert(tree, 20)
    bst_insert(tree, 40)
    bst_insert(tree, 60)
    bst_insert(tree, 80)
    
    assert_eq_int(tree.size, 7) fr fr Test search
    assert_true(bst_search(tree, 50))
    assert_true(bst_search(tree, 30))
    assert_true(bst_search(tree, 70))
    assert_true(bst_search(tree, 20))
    assert_false(bst_search(tree, 25))
    assert_false(bst_search(tree, 100))
    
    vibez.spill("BST tests passed")
}

slay test_avl_tree() {
    test_start("AVL Tree Operations")
    
    sus tree := new_avl<normie>() fr fr Test insert (should maintain balance)
    avl_insert(tree, 10)
    avl_insert(tree, 20)
    avl_insert(tree, 30)
    avl_insert(tree, 40)
    avl_insert(tree, 50)
    avl_insert(tree, 25)
    
    assert_eq_int(tree.size, 6) fr fr Test that tree is balanced (height should be reasonable)
    sus root_height := avl_height(tree.root)
    assert_true(root_height <= 4) fr fr For 6 nodes, height should be <= 4 fr fr Test balance factors are within [-1, 1]
    sus root_balance := avl_balance_factor(tree.root)
    assert_true(root_balance >= -1 && root_balance <= 1)
    
    vibez.spill("AVL Tree tests passed")
}

slay test_priority_queue() {
    test_start("Priority Queue Operations")
    
    sus pq := new_priority_queue<normie>() fr fr Test insert and extract (max heap)
    pq_insert(pq, 10)
    pq_insert(pq, 30)
    pq_insert(pq, 20)
    pq_insert(pq, 40)
    pq_insert(pq, 5) fr fr Test extract max
    sus max1, success1 := pq_extract_max(pq)
    assert_true(success1)
    assert_eq_int(max1, 40)
    
    sus max2, success2 := pq_extract_max(pq)
    assert_true(success2)
    assert_eq_int(max2, 30)
    
    sus max3, success3 := pq_extract_max(pq)
    assert_true(success3)
    assert_eq_int(max3, 20)
    
    sus max4, success4 := pq_extract_max(pq)
    assert_true(success4)
    assert_eq_int(max4, 10)
    
    sus max5, success5 := pq_extract_max(pq)
    assert_true(success5)
    assert_eq_int(max5, 5) fr fr Test empty queue
    sus max_empty, success_empty := pq_extract_max(pq)
    assert_false(success_empty)
    
    vibez.spill("Priority Queue tests passed")
}

slay test_collections_integration() {
    test_start("Collections Integration Test") fr fr Test using multiple collections together
    sus map := new_hashmap<tea, normie>(10)
    sus list := new_arraylist<tea>(5)
    sus stack := new_stack<normie>() fr fr Build data in list
    arraylist_add(list, "apple")
    arraylist_add(list, "banana")
    arraylist_add(list, "cherry") fr fr Map fruits to lengths and push lengths to stack
    bestie i := 0; i < list.size; i++ {
        sus fruit, exists := arraylist_get(list, i)
        ayo (exists) {
            sus length := len(fruit)
            hashmap_put(map, fruit, length)
            stack_push(stack, length)
        }
    } fr fr Verify map contents
    sus apple_len, apple_exists := hashmap_get(map, "apple")
    assert_true(apple_exists)
    assert_eq_int(apple_len, 5)
    
    sus banana_len, banana_exists := hashmap_get(map, "banana")
    assert_true(banana_exists)
    assert_eq_int(banana_len, 6) fr fr Verify stack contents (LIFO)
    sus len1, success1 := stack_pop(stack)
    assert_true(success1)
    assert_eq_int(len1, 6) fr fr cherry length
    
    sus len2, success2 := stack_pop(stack)
    assert_true(success2)
    assert_eq_int(len2, 6) fr fr banana length
    
    sus len3, success3 := stack_pop(stack)
    assert_true(success3)
    assert_eq_int(len3, 5) fr fr apple length
    
    vibez.spill("Collections integration tests passed")
}

slay test_collections_performance() {
    test_start("Collections Performance Test") fr fr Test large dataset operations
    sus large_map := new_hashmap<normie, normie>(1000)
    sus large_list := new_arraylist<normie>(1000)
    sus large_tree := new_avl<normie>() fr fr Insert 1000 elements
    bestie i := 0; i < 1000; i++ {
        hashmap_put(large_map, i, i * 2)
        arraylist_add(large_list, i * 3)
        avl_insert(large_tree, i)
    } fr fr Verify random access performance
    sus map_val, map_exists := hashmap_get(large_map, 500)
    assert_true(map_exists)
    assert_eq_int(map_val, 1000)
    
    sus list_val, list_exists := arraylist_get(large_list, 500)
    assert_true(list_exists)
    assert_eq_int(list_val, 1500) fr fr Verify tree balance after many insertions
    sus tree_height := avl_height(large_tree.root)
    assert_true(tree_height <= 12) fr fr log2(1000) ≈ 10, so height should be reasonable
    
    vibez.spill("Performance tests passed - 1000 elements processed")
}

slay test_generic_collections() {
    test_start("Generic Collections Test") fr fr Test collections with different types
    sus string_list := new_arraylist<tea>(5)
    sus int_stack := new_stack<normie>()
    sus string_set := new_set<tea>() fr fr String operations
    arraylist_add(string_list, "hello")
    arraylist_add(string_list, "world")
    arraylist_add(string_list, "generics")
    
    sus str_val, str_exists := arraylist_get(string_list, 1)
    assert_true(str_exists)
    assert_eq_string(str_val, "world") fr fr Integer operations
    stack_push(int_stack, 42)
    stack_push(int_stack, 99)
    
    sus int_val, int_exists := stack_pop(int_stack)
    assert_true(int_exists)
    assert_eq_int(int_val, 99) fr fr Set operations with strings
    set_add(string_set, "unique")
    set_add(string_set, "values")
    set_add(string_set, "unique") fr fr Duplicate
    
    assert_true(set_contains(string_set, "unique"))
    assert_true(set_contains(string_set, "values"))
    assert_false(set_contains(string_set, "missing"))
    
    vibez.spill("Generic collections tests passed")
}

slay run_all_tests() {
    vibez.spill("=== Advanced Collections Test Suite ===")
    
    test_hashmap()
    test_arraylist()
    test_linkedlist()
    test_stack()
    test_queue()
    test_set()
    test_bst()
    test_avl_tree()
    test_priority_queue()
    test_collections_integration()
    test_collections_performance()
    test_generic_collections()
    
    vibez.spill("\n=== Performance Benchmark ===")
    benchmark_collections()
    
    print_test_summary()
    vibez.spill("All advanced collections tests completed!")
}

fr fr Run all tests
run_all_tests()
