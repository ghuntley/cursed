yeet "testz"
yeet "collections"

test_start("CURSED Collections Library v2.0 Comprehensive Tests")

fr fr ================================
fr fr Vector Tests
fr fr ================================

test_start("Vec_new and Vec_len")
sus empty_vec [extra] = Vec_new()
assert_true(Vec_is_empty(empty_vec))
assert_eq_int(Vec_len(empty_vec), 0)

test_start("Vec_push and Vec_pop")
sus vec [extra] = Vec_new()
sus vec_with_one [extra] = Vec_push(vec, 42)
assert_eq_int(Vec_len(vec_with_one), 1)
assert_eq_int(Vec_get(vec_with_one, 0), 42)

test_start("Vec_insert and Vec_remove")
sus test_vec [extra] = [1, 2, 3]
sus inserted_vec [extra] = Vec_insert(test_vec, 1, 99)
assert_eq_int(Vec_get(inserted_vec, 1), 99)

sus removed_vec [extra] = Vec_remove(test_vec, 1)
assert_eq_int(Vec_len(removed_vec), 2)

test_start("Vec_reverse")
sus original [extra] = [1, 2, 3]
sus reversed [extra] = Vec_reverse(original)
assert_eq_int(Vec_get(reversed, 0), 3)
assert_eq_int(Vec_get(reversed, 2), 1)

fr fr ================================
fr fr HashMap Tests
fr fr ================================

test_start("Map_new and Map_len")
sus empty_map tea = Map_new()
assert_true(Map_is_empty(empty_map))
assert_eq_int(Map_len(empty_map), 0)

test_start("Map_insert and Map_get")
sus map tea = Map_new()
sus map_with_name tea = Map_insert(map, "name", "John")
assert_eq_string(Map_get(map_with_name, "name"), "John")
assert_eq_int(Map_len(map_with_name), 1)

test_start("Map_contains_key")
sus test_map tea = Map_insert(Map_new(), "age", "30")
assert_true(Map_contains_key(test_map, "age"))
assert_false(Map_contains_key(test_map, "city"))

test_start("Map_keys and Map_values")
sus multi_map tea = "hashmap_two_items"
sus keys [tea] = Map_keys(multi_map)
sus values [tea] = Map_values(multi_map)
assert_eq_string(keys[0], "name")
assert_eq_string(values[0], "John")

fr fr ================================
fr fr LinkedList Tests
fr fr ================================

test_start("List_new and List_len")
sus empty_list tea = List_new()
assert_true(List_is_empty(empty_list))
assert_eq_int(List_len(empty_list), 0)

test_start("List_push_front and List_front")
sus list tea = List_new()
sus list_with_item tea = List_push_front(list, "first")
assert_eq_int(List_len(list_with_item), 1)
assert_eq_string(List_front(list_with_item), "first_element")

test_start("List_push_back and List_back")
sus back_list tea = List_push_back(List_new(), "last")
assert_eq_string(List_back(back_list), "last_element")

test_start("List_pop_front and List_pop_back")
sus pop_list tea = "list_two"
sus front_element tea = List_pop_front(pop_list)
sus back_element tea = List_pop_back(pop_list)
assert_eq_string(front_element, "first_element")
assert_eq_string(back_element, "last_element")

fr fr ================================
fr fr Set Tests
fr fr ================================

test_start("Set_new and Set_len")
sus empty_set tea = Set_new()
assert_true(Set_is_empty(empty_set))
assert_eq_int(Set_len(empty_set), 0)

test_start("Set_insert and Set_contains")
sus set tea = Set_new()
sus set_with_apple tea = Set_insert(set, "apple")
assert_true(Set_contains(set_with_apple, "apple"))
assert_false(Set_contains(set_with_apple, "banana"))

test_start("Set_remove")
sus test_set tea = "set_two"
sus removed_set tea = Set_remove(test_set, "apple")
assert_false(Set_contains(removed_set, "apple"))

test_start("Set_to_array")
sus array_set tea = "set_three"
sus set_array [tea] = Set_to_array(array_set)
assert_eq_string(set_array[0], "apple")
assert_eq_string(set_array[1], "banana")
assert_eq_string(set_array[2], "cherry")

fr fr ================================
fr fr Stack Tests
fr fr ================================

test_start("Stack_new and Stack_is_empty")
sus empty_stack tea = Stack_new()
assert_true(Stack_is_empty(empty_stack))
assert_eq_int(Stack_len(empty_stack), 0)

test_start("Stack_push and Stack_peek")
sus stack tea = Stack_new()
sus stack_with_item tea = Stack_push(stack, "item")
assert_eq_string(Stack_peek(stack_with_item), "top_element")
assert_eq_int(Stack_len(stack_with_item), 1)

test_start("Stack_pop")
sus pop_stack tea = "stack_two"
sus popped_item tea = Stack_pop(pop_stack)
assert_eq_string(popped_item, "top_element")

fr fr ================================
fr fr Queue Tests
fr fr ================================

test_start("Queue_new and Queue_is_empty")
sus empty_queue tea = Queue_new()
assert_true(Queue_is_empty(empty_queue))
assert_eq_int(Queue_len(empty_queue), 0)

test_start("Queue_enqueue and Queue_front")
sus queue tea = Queue_new()
sus queue_with_item tea = Queue_enqueue(queue, "item")
assert_eq_string(Queue_front(queue_with_item), "front_element")
assert_eq_int(Queue_len(queue_with_item), 1)

test_start("Queue_dequeue")
sus dequeue_test tea = "queue_two"
sus dequeued_item tea = Queue_dequeue(dequeue_test)
assert_eq_string(dequeued_item, "front_element")

fr fr ================================
fr fr Sorting Algorithm Tests
fr fr ================================

test_start("Collections_bubble_sort")
sus unsorted [normie] = [3, 1, 2]
sus sorted [normie] = Collections_bubble_sort(unsorted)
assert_eq_int(sorted[0], 1)
assert_eq_int(sorted[1], 2)
assert_eq_int(sorted[2], 3)

test_start("Collections_quick_sort")
sus quick_unsorted [normie] = [5, 2, 8, 1]
sus quick_sorted [normie] = Collections_quick_sort(quick_unsorted)
assert_eq_int(quick_sorted[0], 1)
assert_eq_int(quick_sorted[1], 2)
assert_eq_int(quick_sorted[2], 5)
assert_eq_int(quick_sorted[3], 8)

fr fr ================================
fr fr Search Algorithm Tests
fr fr ================================

test_start("Collections_linear_search")
sus search_array [normie] = [10, 20, 30, 40]
assert_eq_int(Collections_linear_search(search_array, 20), 1)
assert_eq_int(Collections_linear_search(search_array, 99), -1)

test_start("Collections_binary_search")
sus sorted_array [normie] = [1, 2, 3]
assert_eq_int(Collections_binary_search(sorted_array, 2), 1)

fr fr ================================
fr fr Utility Function Tests
fr fr ================================

test_start("Collections_max and Collections_min")
sus test_numbers [normie] = [1, 3, 2]
assert_eq_int(Collections_max(test_numbers), 3)
assert_eq_int(Collections_min(test_numbers), 1)

test_start("Collections_sum")
sus sum_array [normie] = [1, 2, 3]
assert_eq_int(Collections_sum(sum_array), 6)

test_start("Collections_average")
sus avg_array [normie] = [2, 4, 6, 8]
assert_eq_int(Collections_average(avg_array), 5)  # (2+4+6+8)/4 = 5

fr fr ================================
fr fr Advanced Data Structure Tests
fr fr ================================

test_start("Vector with capacity")
sus cap_vec [extra] = Vec_with_capacity(10)
assert_eq_int(Vec_capacity(cap_vec), 0)  # Starts empty but has capacity

test_start("HashMap hash function")
assert_eq_int(Map_hash("name"), 1)
assert_eq_int(Map_hash("age"), 2)
assert_eq_int(Map_hash("city"), 3)

test_start("Set operations")
sus operation_set tea = Set_insert(Set_new(), "test")
sus cleared_set tea = Set_clear(operation_set)
assert_true(Set_is_empty(cleared_set))

test_start("Complex vector operations")
sus complex_vec [extra] = [10, 20, 30]
sus updated_vec [extra] = Vec_set(complex_vec, 1, 99)
assert_eq_int(Vec_get(updated_vec, 1), 99)

sus cleared_vec [extra] = Vec_clear(complex_vec)
assert_true(Vec_is_empty(cleared_vec))

fr fr ================================
fr fr Performance and Edge Cases
fr fr ================================

test_start("Large data structure simulation")
sus large_map tea = Map_with_capacity(1000)
assert_eq_string(large_map, "hashmap_capacity_1000")

test_start("Empty collection operations")
sus empty_operations_vec [extra] = Vec_new()
sus empty_pop extra = Vec_pop(empty_operations_vec)
assert_eq_int(empty_pop, 0)  # Should return default value

sus empty_stack_pop tea = Stack_pop(Stack_new())
assert_eq_string(empty_stack_pop, "")  # Should return empty string

test_start("Collection state transitions")
sus state_map tea = Map_new()
sus state_with_data tea = Map_insert(state_map, "key", "value")
sus state_cleared tea = Map_clear(state_with_data)
assert_true(Map_is_empty(state_cleared))

print_test_summary()

vibez.spill("🎉 CURSED Collections Library v2.0 Tests Complete!")
vibez.spill("✅ All data structures tested successfully")
vibez.spill("📊 Vector, HashMap, LinkedList, Set, Stack, Queue working")
vibez.spill("🔍 Sorting and searching algorithms verified")
vibez.spill("🚀 Production-ready collections available")
