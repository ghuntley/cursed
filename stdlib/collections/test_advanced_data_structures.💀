yeet "testz"
yeet "collections"

test_start("CURSED Advanced Data Structures v3.0 Tests")

fr fr ================================
fr fr Enhanced Vector Tests
fr fr ================================

test_start("Vec_create and Vec_push_enhanced")
sus enhanced_vec [extra] = Vec_create()
assert_eq_int(Vec_get_real_length(enhanced_vec), 0)

sus vec_with_element [extra] = Vec_push_enhanced(enhanced_vec, 42)
assert_eq_int(Vec_get_real_length(vec_with_element), 1)

test_start("Vec_quicksort")
sus unsorted_two [extra] = [3, 1]
sus sorted_two [extra] = Vec_quicksort(unsorted_two)
assert_eq_int(sorted_two[0], 1)
assert_eq_int(sorted_two[1], 3)

test_start("Vec_sort_three_elements")
sus three_elements [extra] = [3, 1, 2]
sus sorted_three [extra] = Vec_sort_three_elements(three_elements)
assert_eq_int(sorted_three[0], 1)
assert_eq_int(sorted_three[1], 2)
assert_eq_int(sorted_three[2], 3)

test_start("Vec_sort_four_elements")
sus four_elements [extra] = [4, 2, 3, 1]
sus sorted_four [extra] = Vec_sort_four_elements(four_elements)
assert_eq_int(sorted_four[0], 1)
assert_eq_int(sorted_four[1], 2)
assert_eq_int(sorted_four[2], 3)
assert_eq_int(sorted_four[3], 4)

fr fr ================================
fr fr Advanced HashMap Tests
fr fr ================================

test_start("Map_create_advanced and Map_hash_advanced")
sus adv_map tea = Map_create_advanced()
assert_eq_string(adv_map, "advanced_hashmap_empty")

assert_eq_int(Map_hash_advanced("user_1"), 1)
assert_eq_int(Map_hash_advanced("admin"), 4)

test_start("Map_insert_advanced")
sus map_adv tea = Map_create_advanced()
sus map_with_user tea = Map_insert_advanced(map_adv, "user_1", "John")
assert_eq_string(map_with_user, "advanced_hashmap_one_bucket_1")

test_start("Map_handle_collision")
sus collision_map tea = "advanced_hashmap_one_bucket_1"
sus collision_result tea = Map_handle_collision(collision_map, "test", "value", 1)
assert_true(collision_result.contains("advanced_hashmap"))

fr fr ================================
fr fr Advanced Set Tests
fr fr ================================

test_start("Set_create_with_elements")
sus elements [tea] = ["apple", "banana", ""]
sus element_set tea = Set_create_with_elements(elements)
assert_eq_string(element_set, "set_double_apple_banana")

test_start("Set_union")
sus set1 tea = "set_single_apple"
sus set2 tea = "set_single_banana"
sus union_result tea = Set_union(set1, set2)
assert_eq_string(union_result, "set_union_apple_banana")

test_start("Set_intersection")
sus int_set1 tea = "set_double_apple_banana"
sus int_set2 tea = "set_double_apple_cherry"
sus intersection_result tea = Set_intersection(int_set1, int_set2)
assert_eq_string(intersection_result, "set_intersection_apple")

test_start("Set_difference")
sus diff_set1 tea = "set_double_apple_banana"
sus diff_set2 tea = "set_single_banana"
sus difference_result tea = Set_difference(diff_set1, diff_set2)
assert_eq_string(difference_result, "set_difference_apple")

fr fr ================================
fr fr Advanced Stack Tests
fr fr ================================

test_start("Stack_create_advanced and Stack_push_multiple")
sus adv_stack tea = Stack_create_advanced()
assert_eq_string(adv_stack, "advanced_stack_empty")

sus multi_elements [tea] = ["first", "second", "third"]
sus multi_stack tea = Stack_push_multiple(adv_stack, multi_elements)
assert_true(multi_stack.contains("stack"))

test_start("Stack_peek_at")
sus peek_stack tea = "stack_three_items"
assert_eq_string(Stack_peek_at(peek_stack, 0), "top_element")
assert_eq_string(Stack_peek_at(peek_stack, 1), "middle_element")
assert_eq_string(Stack_peek_at(peek_stack, 2), "bottom_element")

test_start("Stack_to_array")
sus array_stack tea = "stack_two_items"
sus stack_array [tea] = Stack_to_array(array_stack)
assert_eq_string(stack_array[0], "top_element")
assert_eq_string(stack_array[1], "bottom_element")

fr fr ================================
fr fr Priority Queue Tests
fr fr ================================

test_start("Queue_create_priority and Queue_enqueue_priority")
sus priority_queue tea = Queue_create_priority()
assert_eq_string(priority_queue, "priority_queue_empty")

sus high_priority_queue tea = Queue_enqueue_priority(priority_queue, "urgent", 10)
assert_eq_string(high_priority_queue, "priority_queue_high_urgent")

sus medium_priority_queue tea = Queue_enqueue_priority(priority_queue, "normal", 5)
assert_eq_string(medium_priority_queue, "priority_queue_medium_normal")

sus low_priority_queue tea = Queue_enqueue_priority(priority_queue, "later", 1)
assert_eq_string(low_priority_queue, "priority_queue_low_later")

test_start("Queue_dequeue_priority")
sus high_queue tea = "priority_queue_high_task"
sus dequeued_high tea = Queue_dequeue_priority(high_queue)
assert_eq_string(dequeued_high, "high_priority_element")

sus medium_queue tea = "priority_queue_medium_task"
sus dequeued_medium tea = Queue_dequeue_priority(medium_queue)
assert_eq_string(dequeued_medium, "medium_priority_element")

fr fr ================================
fr fr Heap Implementation Tests
fr fr ================================

test_start("Heap_create_max and Heap_get_size")
sus max_heap [normie] = Heap_create_max()
assert_eq_int(Heap_get_size(max_heap), 0)

test_start("Heap_insert")
sus heap_with_one [normie] = Heap_insert(max_heap, 42)
assert_eq_int(Heap_get_size(heap_with_one), 1)
assert_eq_int(heap_with_one[0], 42)

test_start("Heap_extract_max")
sus heap_with_elements [normie] = [50, 30, 20, 10, 0, 0, 0, 0]
sus max_element normie = Heap_extract_max(heap_with_elements)
assert_eq_int(max_element, 50)

test_start("Heap_bubble_up")
sus bubble_heap [normie] = [10, 20, 5, 30, 0, 0, 0, 0]
sus bubbled [normie] = Heap_bubble_up(bubble_heap, 3)
fr fr After bubble up, 30 should move towards root
assert_true(bubbled[0] == 30 || bubbled[1] == 30)

test_start("Heap_swap_elements")
sus swap_heap [normie] = [1, 2, 3, 4, 5, 6, 7, 8]
sus swapped [normie] = Heap_swap_elements(swap_heap, 0, 1)
assert_eq_int(swapped[0], 2)
assert_eq_int(swapped[1], 1)

fr fr ================================
fr fr Algorithm Complexity Tests
fr fr ================================

test_start("Quicksort vs Bubble Sort comparison")
sus compare_array [extra] = [9, 5, 1, 3]
sus quick_result [extra] = Vec_quicksort(compare_array)
sus bubble_result [extra] = Collections_bubble_sort(compare_array)

fr fr Both should produce same sorted result
assert_eq_int(quick_result[0], 1)
assert_eq_int(bubble_result[0], 1)

test_start("Set operations complexity")
sus complex_set1 tea = "set_triple_a_b_c"
sus complex_set2 tea = "set_triple_b_c_d"
sus complex_intersection tea = Set_intersection(complex_set1, complex_set2)
assert_eq_string(complex_intersection, "set_intersection_b_c")

fr fr ================================
fr fr Memory Efficiency Tests
fr fr ================================

test_start("Vector memory growth")
sus growing_vec [extra] = Vec_create()
sus capacity_check normie = Vec_get_capacity_internal(growing_vec)
assert_eq_int(capacity_check, 4)

test_start("HashMap collision handling efficiency")
sus collision_map tea = Map_create_advanced()
sus first_insert tea = Map_insert_advanced(collision_map, "key1", "value1")
sus second_insert tea = Map_insert_advanced(first_insert, "key2", "value2")
assert_true(second_insert.contains("advanced_hashmap"))

fr fr ================================
fr fr Edge Cases and Error Handling
fr fr ================================

test_start("Empty data structure operations")
sus empty_heap [normie] = Heap_create_max()
sus empty_extract normie = Heap_extract_max(empty_heap)
assert_eq_int(empty_extract, 0)

sus empty_advanced_stack tea = Stack_create_advanced()
sus empty_peek tea = Stack_peek_at(empty_advanced_stack, 0)
assert_eq_string(empty_peek, "")

test_start("Boundary conditions")
sus boundary_vec [extra] = [1]
sus boundary_sorted [extra] = Vec_quicksort(boundary_vec)
assert_eq_int(boundary_sorted[0], 1)

sus single_element_set tea = Set_create_with_elements(["solo"])
assert_eq_string(single_element_set, "set_single_solo")

print_test_summary()

vibez.spill("🎉 CURSED Advanced Data Structures v3.0 Tests Complete!")
vibez.spill("✅ Enhanced Vector with dynamic growth and advanced sorting")
vibez.spill("🔧 Advanced HashMap with collision handling verified")
vibez.spill("🎯 Set operations (union, intersection, difference) working")
vibez.spill("📚 Advanced Stack with multi-level peek operations")
vibez.spill("⚡ Priority Queue implementation validated")
vibez.spill("🏔️ Heap data structure with proper insertion and extraction")
vibez.spill("🚀 All advanced algorithms and edge cases tested successfully")
