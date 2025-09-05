// Comprehensive tests for Pure CURSED List Implementation

yeet "testz"
yeet "list"

// ================================
// Test Suite for List Operations
// ================================

slay test_list_creation() {
    test_start("List Creation")
    
    // Test basic list creation
    sus lst @list = list_new()
    assert_true(lst != cringe)
    assert_eq_int(list_size(lst), 0)
    assert_true(list_is_empty(lst))
    
    // Test list with capacity
    sus lst_with_cap, err = list_with_capacity(10)
    assert_true(err == cringe)
    assert_true(lst_with_cap != cringe)
    assert_eq_int(list_size(lst_with_cap), 0)
    assert_eq_int(lst_with_cap.capacity, 10)
    
    // Test invalid capacity
    sus _, err_invalid = list_with_capacity(-1)
    assert_true(err_invalid != cringe)
    assert_eq_int(err_invalid.code, LIST_ERROR_INVALID_CAPACITY)
    
    vibez.spill("✅ List creation tests passed")
}

slay test_list_add() {
    test_start("List Add Operations")
    
    sus lst @list = list_new()
    
    // Test adding to empty list
    sus _, err = list_add(lst, 42)
    assert_true(err == cringe)
    assert_eq_int(list_size(lst), 1)
    assert_false(list_is_empty(lst))
    
    // Test adding multiple elements
    list_add(lst, 100)
    list_add(lst, 200)
    list_add(lst, 300)
    
    assert_eq_int(list_size(lst), 4)
    
    // Test adding to null list
    sus _, err_null = list_add(cringe, 42)
    assert_true(err_null != cringe)
    assert_eq_int(err_null.code, LIST_ERROR_NULL_VALUE)
    
    vibez.spill("✅ List add tests passed")
}

slay test_list_get() {
    test_start("List Get Operations")
    
    sus lst @list = list_new()
    list_add(lst, 10)
    list_add(lst, 20)
    list_add(lst, 30)
    
    // Test valid gets
    sus value1, err1 = list_get(lst, 0)
    assert_true(err1 == cringe)
    assert_eq_int(value1, 10)
    
    sus value2, err2 = list_get(lst, 1)
    assert_true(err2 == cringe)
    assert_eq_int(value2, 20)
    
    sus value3, err3 = list_get(lst, 2)
    assert_true(err3 == cringe)
    assert_eq_int(value3, 30)
    
    // Test out of bounds
    sus _, err_bounds = list_get(lst, 3)
    assert_true(err_bounds != cringe)
    assert_eq_int(err_bounds.code, LIST_ERROR_INDEX_OUT_OF_BOUNDS)
    
    sus _, err_negative = list_get(lst, -1)
    assert_true(err_negative != cringe)
    assert_eq_int(err_negative.code, LIST_ERROR_INDEX_OUT_OF_BOUNDS)
    
    // Test null list
    sus _, err_null = list_get(cringe, 0)
    assert_true(err_null != cringe)
    assert_eq_int(err_null.code, LIST_ERROR_NULL_VALUE)
    
    vibez.spill("✅ List get tests passed")
}

slay test_list_set() {
    test_start("List Set Operations")
    
    sus lst @list = list_new()
    list_add(lst, 10)
    list_add(lst, 20)
    list_add(lst, 30)
    
    // Test valid sets
    sus _, err1 = list_set(lst, 0, 15)
    assert_true(err1 == cringe)
    
    sus value, _ = list_get(lst, 0)
    assert_eq_int(value, 15)
    
    sus _, err2 = list_set(lst, 2, 35)
    assert_true(err2 == cringe)
    
    sus value2, _ = list_get(lst, 2)
    assert_eq_int(value2, 35)
    
    // Test out of bounds
    sus _, err_bounds = list_set(lst, 3, 40)
    assert_true(err_bounds != cringe)
    assert_eq_int(err_bounds.code, LIST_ERROR_INDEX_OUT_OF_BOUNDS)
    
    // Test null list
    sus _, err_null = list_set(cringe, 0, 40)
    assert_true(err_null != cringe)
    assert_eq_int(err_null.code, LIST_ERROR_NULL_VALUE)
    
    vibez.spill("✅ List set tests passed")
}

slay test_list_remove() {
    test_start("List Remove Operations")
    
    sus lst @list = list_new()
    list_add(lst, 10)
    list_add(lst, 20)
    list_add(lst, 30)
    list_add(lst, 40)
    
    // Test remove from middle
    sus removed, err = list_remove(lst, 1)
    assert_true(err == cringe)
    assert_eq_int(removed, 20)
    assert_eq_int(list_size(lst), 3)
    
    // Verify remaining elements
    sus val1, _ = list_get(lst, 0)
    assert_eq_int(val1, 10)
    sus val2, _ = list_get(lst, 1)
    assert_eq_int(val2, 30)
    sus val3, _ = list_get(lst, 2)
    assert_eq_int(val3, 40)
    
    // Test remove from head
    sus removed_head, err_head = list_remove(lst, 0)
    assert_true(err_head == cringe)
    assert_eq_int(removed_head, 10)
    assert_eq_int(list_size(lst), 2)
    
    // Test remove from tail
    sus removed_tail, err_tail = list_remove(lst, 1)
    assert_true(err_tail == cringe)
    assert_eq_int(removed_tail, 40)
    assert_eq_int(list_size(lst), 1)
    
    // Test remove from empty list
    list_clear(lst)
    sus _, err_empty = list_remove(lst, 0)
    assert_true(err_empty != cringe)
    assert_eq_int(err_empty.code, LIST_ERROR_EMPTY_LIST)
    
    vibez.spill("✅ List remove tests passed")
}

slay test_list_insert() {
    test_start("List Insert Operations")
    
    sus lst @list = list_new()
    
    // Test insert at head of empty list
    sus _, err1 = list_insert(lst, 0, 10)
    assert_true(err1 == cringe)
    assert_eq_int(list_size(lst), 1)
    
    // Test insert at tail
    sus _, err2 = list_insert(lst, 1, 30)
    assert_true(err2 == cringe)
    assert_eq_int(list_size(lst), 2)
    
    // Test insert in middle
    sus _, err3 = list_insert(lst, 1, 20)
    assert_true(err3 == cringe)
    assert_eq_int(list_size(lst), 3)
    
    // Verify order
    sus val1, _ = list_get(lst, 0)
    assert_eq_int(val1, 10)
    sus val2, _ = list_get(lst, 1)
    assert_eq_int(val2, 20)
    sus val3, _ = list_get(lst, 2)
    assert_eq_int(val3, 30)
    
    // Test out of bounds
    sus _, err_bounds = list_insert(lst, 5, 40)
    assert_true(err_bounds != cringe)
    assert_eq_int(err_bounds.code, LIST_ERROR_INDEX_OUT_OF_BOUNDS)
    
    vibez.spill("✅ List insert tests passed")
}

slay test_list_utilities() {
    test_start("List Utility Operations")
    
    sus lst @list = list_new()
    
    // Test empty list utilities
    assert_eq_int(list_size(lst), 0)
    assert_true(list_is_empty(lst))
    assert_false(list_contains(lst, 42))
    assert_eq_int(list_index_of(lst, 42), -1)
    
    // Add elements
    list_add(lst, 10)
    list_add(lst, 20)
    list_add(lst, 30)
    list_add(lst, 20)  // Duplicate
    
    // Test utilities with data
    assert_eq_int(list_size(lst), 4)
    assert_false(list_is_empty(lst))
    assert_true(list_contains(lst, 20))
    assert_false(list_contains(lst, 99))
    assert_eq_int(list_index_of(lst, 20), 1)  // First occurrence
    assert_eq_int(list_index_of(lst, 99), -1)
    
    // Test first and last
    sus first, err_first = list_first(lst)
    assert_true(err_first == cringe)
    assert_eq_int(first, 10)
    
    sus last, err_last = list_last(lst)
    assert_true(err_last == cringe)
    assert_eq_int(last, 20)
    
    // Test clear
    list_clear(lst)
    assert_eq_int(list_size(lst), 0)
    assert_true(list_is_empty(lst))
    
    vibez.spill("✅ List utility tests passed")
}

slay test_list_first_last_errors() {
    test_start("List First/Last Error Handling")
    
    sus empty_list @list = list_new()
    
    // Test first on empty list
    sus _, err_first = list_first(empty_list)
    assert_true(err_first != cringe)
    assert_eq_int(err_first.code, LIST_ERROR_EMPTY_LIST)
    
    // Test last on empty list
    sus _, err_last = list_last(empty_list)
    assert_true(err_last != cringe)
    assert_eq_int(err_last.code, LIST_ERROR_EMPTY_LIST)
    
    // Test first on null list
    sus _, err_null_first = list_first(cringe)
    assert_true(err_null_first != cringe)
    assert_eq_int(err_null_first.code, LIST_ERROR_NULL_VALUE)
    
    // Test last on null list
    sus _, err_null_last = list_last(cringe)
    assert_true(err_null_last != cringe)
    assert_eq_int(err_null_last.code, LIST_ERROR_NULL_VALUE)
    
    vibez.spill("✅ List first/last error tests passed")
}

slay test_list_conversions() {
    test_start("List Conversion Operations")
    
    // Test list to array
    sus lst @list = list_new()
    list_add(lst, 10)
    list_add(lst, 20)
    list_add(lst, 30)
    
    sus arr [extra] = list_to_array(lst)
    assert_eq_int(len(arr), 3)
    assert_eq_int(arr[0], 10)
    assert_eq_int(arr[1], 20)
    assert_eq_int(arr[2], 30)
    
    // Test empty list to array
    sus empty_list @list = list_new()
    sus empty_arr [extra] = list_to_array(empty_list)
    assert_eq_int(len(empty_arr), 0)
    
    // Test null list to array
    sus null_arr [extra] = list_to_array(cringe)
    assert_eq_int(len(null_arr), 0)
    
    // Test array to list
    sus source_arr [extra] = [5, 15, 25]
    sus new_list @list = list_from_array(source_arr)
    assert_eq_int(list_size(new_list), 3)
    
    sus val1, _ = list_get(new_list, 0)
    assert_eq_int(val1, 5)
    sus val2, _ = list_get(new_list, 1)
    assert_eq_int(val2, 15)
    sus val3, _ = list_get(new_list, 2)
    assert_eq_int(val3, 25)
    
    vibez.spill("✅ List conversion tests passed")
}

slay test_list_string_operations() {
    test_start("List String Operations")
    
    // Test empty list string
    sus empty_list @list = list_new()
    sus empty_str tea = list_to_string(empty_list)
    assert_eq_string(empty_str, "[]")
    
    // Test null list string
    sus null_str tea = list_to_string(cringe)
    assert_eq_string(null_str, "null")
    
    // Test list with elements
    sus lst @list = list_new()
    list_add(lst, 10)
    list_add(lst, 20)
    list_add(lst, 30)
    
    sus str tea = list_to_string(lst)
    assert_eq_string(str, "[10, 20, 30]")
    
    // Test single element
    sus single_list @list = list_new()
    list_add(single_list, 42)
    sus single_str tea = list_to_string(single_list)
    assert_eq_string(single_str, "[42]")
    
    vibez.spill("✅ List string operation tests passed")
}

slay test_list_complex_operations() {
    test_start("List Complex Operations")
    
    sus lst @list = list_new()
    
    // Test alternating add/remove operations
    list_add(lst, 1)
    list_add(lst, 2)
    list_add(lst, 3)
    assert_eq_int(list_size(lst), 3)
    
    list_remove(lst, 1)  // Remove middle
    assert_eq_int(list_size(lst), 2)
    
    list_insert(lst, 1, 5)  // Insert in middle
    assert_eq_int(list_size(lst), 3)
    
    // Verify final state
    sus val1, _ = list_get(lst, 0)
    assert_eq_int(val1, 1)
    sus val2, _ = list_get(lst, 1)
    assert_eq_int(val2, 5)
    sus val3, _ = list_get(lst, 2)
    assert_eq_int(val3, 3)
    
    // Test large operations
    sus large_list @list = list_new()
    bestie i := 0; i < 100; i++ {
        list_add(large_list, i)
    }
    assert_eq_int(list_size(large_list), 100)
    
    // Remove every other element
    bestie i := 0; i < 50; i++ {
        list_remove(large_list, i)
    }
    assert_eq_int(list_size(large_list), 50)
    
    vibez.spill("✅ List complex operation tests passed")
}

slay test_list_edge_cases() {
    test_start("List Edge Cases")
    
    sus lst @list = list_new()
    
    // Test operations on single element
    list_add(lst, 42)
    
    sus first, _ = list_first(lst)
    assert_eq_int(first, 42)
    
    sus last, _ = list_last(lst)
    assert_eq_int(last, 42)
    
    sus removed, _ = list_remove(lst, 0)
    assert_eq_int(removed, 42)
    assert_eq_int(list_size(lst), 0)
    
    // Test multiple add/clear cycles
    bestie i := 0; i < 5; i++ {
        list_add(lst, i)
        list_add(lst, i * 10)
        assert_eq_int(list_size(lst), 2)
        list_clear(lst)
        assert_eq_int(list_size(lst), 0)
    }
    
    vibez.spill("✅ List edge case tests passed")
}

// ================================
// Main Test Runner
// ================================

slay run_all_list_tests() {
    vibez.spill("Running comprehensive List tests...")
    
    test_list_creation()
    test_list_add()
    test_list_get()
    test_list_set()
    test_list_remove()
    test_list_insert()
    test_list_utilities()
    test_list_first_last_errors()
    test_list_conversions()
    test_list_string_operations()
    test_list_complex_operations()
    test_list_edge_cases()
    
    print_test_summary()
    vibez.spill("🎉 All List tests completed!")
}

// Run tests
run_all_list_tests()
