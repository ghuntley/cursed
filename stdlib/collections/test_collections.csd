yeet "testz"
yeet "collections"

fr fr ========================================
fr fr CURSED Collections Library Test Suite
fr fr ========================================

slay test_array_operations() {
    testz.test_start("Array Operations")
    
    fr fr Test array creation
    sus arr [normie] = array_new()
    testz.assert_true(array_is_empty(arr))
    testz.assert_eq_int(array_len(arr), 0)
    
    fr fr Test array push
    arr = array_push(arr, 1)
    arr = array_push(arr, 2)
    arr = array_push(arr, 3)
    testz.assert_eq_int(array_len(arr), 3)
    testz.assert_false(array_is_empty(arr))
    
    fr fr Test array get
    testz.assert_eq_int(array_get(arr, 0), 1)
    testz.assert_eq_int(array_get(arr, 1), 2)
    testz.assert_eq_int(array_get(arr, 2), 3)
    
    fr fr Test array set
    arr = array_set(arr, 1, 5)
    testz.assert_eq_int(array_get(arr, 1), 5)
    
    fr fr Test array pop
    sus popped normie = array_pop(arr)
    testz.assert_eq_int(popped, 3)
    testz.assert_eq_int(array_len(arr), 2)
    
    fr fr Test array insert
    arr = array_insert(arr, 1, 4)
    testz.assert_eq_int(array_len(arr), 3)
    testz.assert_eq_int(array_get(arr, 1), 4)
    
    fr fr Test array remove
    sus removed normie = array_remove(arr, 1)
    testz.assert_eq_int(removed, 4)
    testz.assert_eq_int(array_len(arr), 2)
}

slay test_array_search() {
    testz.test_start("Array Search Operations")
    
    fr fr Setup test array
    sus arr [normie] = array_new()
    arr = array_push(arr, 10)
    arr = array_push(arr, 20)
    arr = array_push(arr, 30)
    arr = array_push(arr, 20)
    
    fr fr Test contains
    testz.assert_true(array_contains(arr, 20))
    testz.assert_true(array_contains(arr, 10))
    testz.assert_false(array_contains(arr, 40))
    
    fr fr Test index of
    testz.assert_eq_int(array_index_of(arr, 20), 1)
    testz.assert_eq_int(array_index_of(arr, 10), 0)
    testz.assert_eq_int(array_index_of(arr, 40), -1)
}

slay test_array_manipulation() {
    testz.test_start("Array Manipulation")
    
    fr fr Test array reverse
    sus arr [normie] = array_new()
    arr = array_push(arr, 1)
    arr = array_push(arr, 2)
    arr = array_push(arr, 3)
    
    arr = array_reverse(arr)
    testz.assert_eq_int(array_get(arr, 0), 3)
    testz.assert_eq_int(array_get(arr, 1), 2)
    testz.assert_eq_int(array_get(arr, 2), 1)
    
    fr fr Test array slice
    sus sliced [normie] = array_slice(arr, 1, 3)
    testz.assert_eq_int(array_len(sliced), 2)
    testz.assert_eq_int(array_get(sliced, 0), 2)
    testz.assert_eq_int(array_get(sliced, 1), 1)
    
    fr fr Test array concat
    sus arr2 [normie] = array_new()
    arr2 = array_push(arr2, 4)
    arr2 = array_push(arr2, 5)
    
    sus concatenated [normie] = array_concat(arr, arr2)
    testz.assert_eq_int(array_len(concatenated), 5)
    testz.assert_eq_int(array_get(concatenated, 3), 4)
    testz.assert_eq_int(array_get(concatenated, 4), 5)
    
    fr fr Test array clear
    arr = array_clear(arr)
    testz.assert_true(array_is_empty(arr))
    testz.assert_eq_int(array_len(arr), 0)
}

slay test_map_operations() {
    testz.test_start("Map Operations")
    
    fr fr Test map creation
    sus m map = map_new()
    testz.assert_true(map_is_empty(m))
    testz.assert_eq_int(map_len(m), 0)
    
    fr fr Test map set and get
    m = map_set(m, "key1", "value1")
    m = map_set(m, "key2", "value2")
    m = map_set(m, "key3", "value3")
    
    testz.assert_eq_int(map_len(m), 3)
    testz.assert_false(map_is_empty(m))
    testz.assert_eq_string(map_get(m, "key1"), "value1")
    testz.assert_eq_string(map_get(m, "key2"), "value2")
    testz.assert_eq_string(map_get(m, "key3"), "value3")
    
    fr fr Test map contains key
    testz.assert_true(map_contains_key(m, "key1"))
    testz.assert_true(map_contains_key(m, "key2"))
    testz.assert_false(map_contains_key(m, "nonexistent"))
    
    fr fr Test map get or default
    testz.assert_eq_string(map_get_or_default(m, "key1", "default"), "value1")
    testz.assert_eq_string(map_get_or_default(m, "nonexistent", "default"), "default")
    
    fr fr Test map remove
    sus removed tea = map_remove(m, "key2")
    testz.assert_eq_string(removed, "value2")
    testz.assert_eq_int(map_len(m), 2)
    testz.assert_false(map_contains_key(m, "key2"))
}

slay test_map_collections() {
    testz.test_start("Map Collections")
    
    fr fr Setup test map
    sus m map = map_new()
    m = map_set(m, "name", "Alice")
    m = map_set(m, "age", "30")
    m = map_set(m, "city", "New York")
    
    fr fr Test map keys
    sus keys [tea] = map_keys(m)
    testz.assert_eq_int(len(keys), 3)
    testz.assert_true(array_contains(keys, "name"))
    testz.assert_true(array_contains(keys, "age"))
    testz.assert_true(array_contains(keys, "city"))
    
    fr fr Test map values
    sus values [tea] = map_values(m)
    testz.assert_eq_int(len(values), 3)
    testz.assert_true(array_contains(values, "Alice"))
    testz.assert_true(array_contains(values, "30"))
    testz.assert_true(array_contains(values, "New York"))
    
    fr fr Test map merge
    sus m2 map = map_new()
    m2 = map_set(m2, "country", "USA")
    m2 = map_set(m2, "age", "31")
    
    sus merged map = map_merge(m, m2)
    testz.assert_eq_int(map_len(merged), 4)
    testz.assert_eq_string(map_get(merged, "country"), "USA")
    testz.assert_eq_string(map_get(merged, "age"), "31")
}

slay test_set_operations() {
    testz.test_start("Set Operations")
    
    fr fr Test set creation
    sus s set = set_new()
    testz.assert_true(set_is_empty(s))
    testz.assert_eq_int(set_len(s), 0)
    
    fr fr Test set add
    s = set_add(s, "apple")
    s = set_add(s, "banana")
    s = set_add(s, "cherry")
    s = set_add(s, "apple")
    
    testz.assert_eq_int(set_len(s), 3)
    testz.assert_false(set_is_empty(s))
    
    fr fr Test set contains
    testz.assert_true(set_contains(s, "apple"))
    testz.assert_true(set_contains(s, "banana"))
    testz.assert_true(set_contains(s, "cherry"))
    testz.assert_false(set_contains(s, "orange"))
    
    fr fr Test set remove
    testz.assert_true(set_remove(s, "banana"))
    testz.assert_eq_int(set_len(s), 2)
    testz.assert_false(set_contains(s, "banana"))
    testz.assert_false(set_remove(s, "nonexistent"))
}

slay test_set_operations_advanced() {
    testz.test_start("Set Advanced Operations")
    
    fr fr Setup test sets
    sus s1 set = set_new()
    s1 = set_add(s1, "a")
    s1 = set_add(s1, "b")
    s1 = set_add(s1, "c")
    
    sus s2 set = set_new()
    s2 = set_add(s2, "b")
    s2 = set_add(s2, "c")
    s2 = set_add(s2, "d")
    
    fr fr Test set union
    sus union_set set = set_union(s1, s2)
    testz.assert_eq_int(set_len(union_set), 4)
    testz.assert_true(set_contains(union_set, "a"))
    testz.assert_true(set_contains(union_set, "b"))
    testz.assert_true(set_contains(union_set, "c"))
    testz.assert_true(set_contains(union_set, "d"))
    
    fr fr Test set intersection
    sus intersection_set set = set_intersection(s1, s2)
    testz.assert_eq_int(set_len(intersection_set), 2)
    testz.assert_true(set_contains(intersection_set, "b"))
    testz.assert_true(set_contains(intersection_set, "c"))
    testz.assert_false(set_contains(intersection_set, "a"))
    testz.assert_false(set_contains(intersection_set, "d"))
    
    fr fr Test set difference
    sus difference_set set = set_difference(s1, s2)
    testz.assert_eq_int(set_len(difference_set), 1)
    testz.assert_true(set_contains(difference_set, "a"))
    testz.assert_false(set_contains(difference_set, "b"))
    testz.assert_false(set_contains(difference_set, "c"))
    
    fr fr Test subset/superset
    sus subset set = set_new()
    subset = set_add(subset, "a")
    subset = set_add(subset, "b")
    
    testz.assert_true(set_is_subset(subset, s1))
    testz.assert_true(set_is_superset(s1, subset))
    testz.assert_false(set_is_subset(s1, subset))
    testz.assert_false(set_is_superset(subset, s1))
}

slay test_queue_operations() {
    testz.test_start("Queue Operations")
    
    fr fr Test queue creation
    sus q queue = queue_new()
    testz.assert_true(queue_is_empty(q))
    testz.assert_eq_int(queue_len(q), 0)
    
    fr fr Test queue enqueue
    q = queue_enqueue(q, "first")
    q = queue_enqueue(q, "second")
    q = queue_enqueue(q, "third")
    
    testz.assert_eq_int(queue_len(q), 3)
    testz.assert_false(queue_is_empty(q))
    
    fr fr Test queue front and back
    testz.assert_eq_string(queue_front(q), "first")
    testz.assert_eq_string(queue_back(q), "third")
    
    fr fr Test queue dequeue
    testz.assert_eq_string(queue_dequeue(q), "first")
    testz.assert_eq_int(queue_len(q), 2)
    testz.assert_eq_string(queue_front(q), "second")
    
    testz.assert_eq_string(queue_dequeue(q), "second")
    testz.assert_eq_string(queue_dequeue(q), "third")
    testz.assert_true(queue_is_empty(q))
}

slay test_stack_operations() {
    testz.test_start("Stack Operations")
    
    fr fr Test stack creation
    sus s stack = stack_new()
    testz.assert_true(stack_is_empty(s))
    testz.assert_eq_int(stack_len(s), 0)
    
    fr fr Test stack push
    s = stack_push(s, "bottom")
    s = stack_push(s, "middle")
    s = stack_push(s, "top")
    
    testz.assert_eq_int(stack_len(s), 3)
    testz.assert_false(stack_is_empty(s))
    
    fr fr Test stack peek
    testz.assert_eq_string(stack_peek(s), "top")
    testz.assert_eq_int(stack_len(s), 3)
    
    fr fr Test stack pop
    testz.assert_eq_string(stack_pop(s), "top")
    testz.assert_eq_int(stack_len(s), 2)
    testz.assert_eq_string(stack_peek(s), "middle")
    
    testz.assert_eq_string(stack_pop(s), "middle")
    testz.assert_eq_string(stack_pop(s), "bottom")
    testz.assert_true(stack_is_empty(s))
}

slay test_utility_functions() {
    testz.test_start("Utility Functions")
    
    fr fr Test range
    sus range_arr [normie] = range(1, 5)
    testz.assert_eq_int(len(range_arr), 4)
    testz.assert_eq_int(range_arr[0], 1)
    testz.assert_eq_int(range_arr[1], 2)
    testz.assert_eq_int(range_arr[2], 3)
    testz.assert_eq_int(range_arr[3], 4)
    
    fr fr Test range with step
    sus range_step_arr [normie] = range_step(0, 10, 2)
    testz.assert_eq_int(len(range_step_arr), 5)
    testz.assert_eq_int(range_step_arr[0], 0)
    testz.assert_eq_int(range_step_arr[1], 2)
    testz.assert_eq_int(range_step_arr[2], 4)
    testz.assert_eq_int(range_step_arr[3], 6)
    testz.assert_eq_int(range_step_arr[4], 8)
    
    fr fr Test unique
    sus arr_with_dupes [normie] = [1, 2, 2, 3, 3, 3, 4]
    sus unique_arr [normie] = unique(arr_with_dupes)
    testz.assert_eq_int(len(unique_arr), 4)
    testz.assert_true(array_contains(unique_arr, 1))
    testz.assert_true(array_contains(unique_arr, 2))
    testz.assert_true(array_contains(unique_arr, 3))
    testz.assert_true(array_contains(unique_arr, 4))
    
    fr fr Test count occurrences
    sus count_arr [normie] = [1, 2, 2, 3, 2, 4, 2]
    testz.assert_eq_int(count_occurrences(count_arr, 2), 4)
    testz.assert_eq_int(count_occurrences(count_arr, 1), 1)
    testz.assert_eq_int(count_occurrences(count_arr, 5), 0)
}

slay test_array_conversion() {
    testz.test_start("Array Conversion")
    
    fr fr Test set to array conversion
    sus s set = set_new()
    s = set_add(s, "x")
    s = set_add(s, "y")
    s = set_add(s, "z")
    
    sus arr [tea] = set_to_array(s)
    testz.assert_eq_int(len(arr), 3)
    testz.assert_true(array_contains(arr, "x"))
    testz.assert_true(array_contains(arr, "y"))
    testz.assert_true(array_contains(arr, "z"))
    
    fr fr Test array to set conversion
    sus arr2 [tea] = ["a", "b", "c", "a", "b"]
    sus s2 set = set_from_array(arr2)
    testz.assert_eq_int(set_len(s2), 3)
    testz.assert_true(set_contains(s2, "a"))
    testz.assert_true(set_contains(s2, "b"))
    testz.assert_true(set_contains(s2, "c"))
}

slay test_collections_edge_cases() {
    testz.test_start("Collections Edge Cases")
    
    fr fr Test empty array operations
    sus empty_arr [normie] = array_new()
    testz.assert_eq_int(array_index_of(empty_arr, 1), -1)
    testz.assert_false(array_contains(empty_arr, 1))
    
    fr fr Test empty map operations
    sus empty_map map = map_new()
    testz.assert_eq_string(map_get_or_default(empty_map, "key", "default"), "default")
    testz.assert_false(map_contains_key(empty_map, "key"))
    
    fr fr Test empty set operations
    sus empty_set set = set_new()
    testz.assert_false(set_contains(empty_set, "item"))
    testz.assert_false(set_remove(empty_set, "item"))
    
    fr fr Test single element collections
    sus single_arr [normie] = array_new()
    single_arr = array_push(single_arr, 42)
    testz.assert_eq_int(array_pop(single_arr), 42)
    testz.assert_true(array_is_empty(single_arr))
}

slay test_collections_clear() {
    testz.test_start("Collections Clear Operations")
    
    fr fr Test array clear
    sus arr [normie] = array_new()
    arr = array_push(arr, 1)
    arr = array_push(arr, 2)
    arr = array_clear(arr)
    testz.assert_true(array_is_empty(arr))
    
    fr fr Test map clear
    sus m map = map_new()
    m = map_set(m, "key", "value")
    m = map_clear(m)
    testz.assert_true(map_is_empty(m))
    
    fr fr Test set clear
    sus s set = set_new()
    s = set_add(s, "item")
    s = set_clear(s)
    testz.assert_true(set_is_empty(s))
    
    fr fr Test queue clear
    sus q queue = queue_new()
    q = queue_enqueue(q, "item")
    q = queue_clear(q)
    testz.assert_true(queue_is_empty(q))
    
    fr fr Test stack clear
    sus st stack = stack_new()
    st = stack_push(st, "item")
    st = stack_clear(st)
    testz.assert_true(stack_is_empty(st))
}

slay run_all_collections_tests() {
    vibez.spill("📦 Running CURSED Collections Library Tests")
    vibez.spill("=========================================")
    
    test_array_operations()
    test_array_search()
    test_array_manipulation()
    test_map_operations()
    test_map_collections()
    test_set_operations()
    test_set_operations_advanced()
    test_queue_operations()
    test_stack_operations()
    test_utility_functions()
    test_array_conversion()
    test_collections_edge_cases()
    test_collections_clear()
    
    testz.print_test_summary()
    damn testz.run_all_tests()
}

fr fr Auto-run tests when this file is executed
run_all_collections_tests()
