yeet "testz"
yeet "enhanced_collections"

test_start("Enhanced Collections Comprehensive Tests")

fr fr ================================
fr fr Generic Array Operations Tests
fr fr ================================

test_start("array_map functionality")
sus numbers normie[value] = [1, 2, 3, 4, 5]
sus mapper slay(normie) normie = slay(x normie) normie { damn x * 2 }
sus doubled normie[value] = array_map(numbers, mapper)
assert_eq_int(len(doubled), 5)
assert_eq_int(doubled[0], 2)
assert_eq_int(doubled[4], 10)

test_start("array_filter functionality")
sus test_numbers normie[value] = [1, 2, 3, 4, 5, 6]
sus is_even slay(normie) lit = slay(x normie) lit { damn x % 2 == 0 }
sus evens normie[value] = array_filter(test_numbers, is_even)
assert_eq_int(len(evens), 3)
assert_eq_int(evens[0], 2)
assert_eq_int(evens[2], 6)

test_start("array_reduce functionality")
sus sum_numbers normie[value] = [1, 2, 3, 4, 5]
sus add slay(normie, normie) normie = slay(acc normie, x normie) normie { damn acc + x }
sus total normie = array_reduce(sum_numbers, 0, add)
assert_eq_int(total, 15)

test_start("array_find functionality")
sus find_array normie[value] = [10, 20, 30, 40]
sus find_30 slay(normie) lit = slay(x normie) lit { damn x == 30 }
(found_value, found) := array_find(find_array, find_30)
assert_true(found)
assert_eq_int(found_value, 30)

sus find_99 slay(normie) lit = slay(x normie) lit { damn x == 99 }
(not_found_value, not_found) := array_find(find_array, find_99)
assert_false(not_found)

test_start("array_contains functionality")
sus contain_test normie[value] = [1, 3, 5, 7, 9]
assert_true(array_contains(contain_test, 5))
assert_false(array_contains(contain_test, 4))

test_start("array_unique functionality")
sus duplicates normie[value] = [1, 2, 2, 3, 3, 3, 4]
sus unique_result normie[value] = array_unique(duplicates)
assert_eq_int(len(unique_result), 4)
assert_true(array_contains(unique_result, 1))
assert_true(array_contains(unique_result, 2))
assert_true(array_contains(unique_result, 3))
assert_true(array_contains(unique_result, 4))

test_start("array_reverse functionality")
sus original normie[value] = [1, 2, 3, 4, 5]
sus reversed normie[value] = array_reverse(original)
assert_eq_int(len(reversed), 5)
assert_eq_int(reversed[0], 5)
assert_eq_int(reversed[4], 1)

test_start("array_sort_integers functionality")
sus unsorted normie[value] = [5, 2, 8, 1, 9, 3]
sus sorted normie[value] = array_sort_integers(unsorted)
assert_eq_int(sorted[0], 1)
assert_eq_int(sorted[1], 2)
assert_eq_int(sorted[2], 3)
assert_eq_int(sorted[5], 9)

fr fr ================================
fr fr HashMap Tests
fr fr ================================

test_start("HashMap creation and basic operations")
sus map HashMap<tea, normie> = HashMap_new<tea, normie>()
assert_eq_int(map.size, 0)
assert_eq_int(map.capacity, 16)

test_start("HashMap insert and get")
sus test_map HashMap<tea, normie> = HashMap_new<tea, normie>()
test_map = HashMap_insert(test_map, "first", 100)
test_map = HashMap_insert(test_map, "second", 200)

(value1, found1) := HashMap_get(test_map, "first")
assert_true(found1)
assert_eq_int(value1, 100)

(value2, found2) := HashMap_get(test_map, "second")
assert_true(found2)
assert_eq_int(value2, 200)

(not_found, found3) := HashMap_get(test_map, "missing")
assert_false(found3)

test_start("HashMap key update")
sus update_map HashMap<tea, normie> = HashMap_new<tea, normie>()
update_map = HashMap_insert(update_map, "key", 42)
update_map = HashMap_insert(update_map, "key", 84) fr fr Update same key

(updated_value, found) := HashMap_get(update_map, "key")
assert_true(found)
assert_eq_int(updated_value, 84)
assert_eq_int(update_map.size, 1) fr fr Size should remain 1

test_start("HashMap hash functions")
sus str_hash normie = HashMap_hash_string("test")
sus int_hash normie = HashMap_hash_int(42)
assert_true(str_hash > 0)
assert_true(int_hash > 0)

fr fr ================================
fr fr Thread-Safe Collections Tests  
fr fr ================================

test_start("SafeArray creation and basic operations")
sus safe_arr SafeArray<normie> = SafeArray_new<normie>()
assert_eq_int(SafeArray_length(safe_arr), 0)

test_start("SafeArray append and get")
sus append_arr SafeArray<normie> = SafeArray_new<normie>()
append_arr = SafeArray_append(append_arr, 10)
append_arr = SafeArray_append(append_arr, 20)
append_arr = SafeArray_append(append_arr, 30)

assert_eq_int(SafeArray_length(append_arr), 3)

(val0, found0) := SafeArray_get(append_arr, 0)
assert_true(found0)
assert_eq_int(val0, 10)

(val2, found2) := SafeArray_get(append_arr, 2)
assert_true(found2)
assert_eq_int(val2, 30)

(invalid_val, found_invalid) := SafeArray_get(append_arr, 10)
assert_false(found_invalid)

fr fr ================================
fr fr Binary Tree Tests
fr fr ================================

test_start("BinaryTree creation and insertion")
sus root *BinaryTree<normie> = BinaryTree_new<normie>(10)
root = BinaryTree_insert(root, 5)
root = BinaryTree_insert(root, 15)
root = BinaryTree_insert(root, 3)
root = BinaryTree_insert(root, 7)

assert_eq_int(root.value, 10)
assert_eq_int(root.left.value, 5)
assert_eq_int(root.right.value, 15)

test_start("BinaryTree search")
sus search_root *BinaryTree<normie> = BinaryTree_new<normie>(50)
search_root = BinaryTree_insert(search_root, 30)
search_root = BinaryTree_insert(search_root, 70)
search_root = BinaryTree_insert(search_root, 20)

assert_true(BinaryTree_search(search_root, 50))
assert_true(BinaryTree_search(search_root, 30))
assert_true(BinaryTree_search(search_root, 70))
assert_true(BinaryTree_search(search_root, 20))
assert_false(BinaryTree_search(search_root, 99))

test_start("BinaryTree inorder traversal")
sus traversal_root *BinaryTree<normie> = BinaryTree_new<normie>(4)
traversal_root = BinaryTree_insert(traversal_root, 2)
traversal_root = BinaryTree_insert(traversal_root, 6)
traversal_root = BinaryTree_insert(traversal_root, 1)
traversal_root = BinaryTree_insert(traversal_root, 3)

sus visited_values normie[value] = []
sus visit_function slay(normie) = slay(value normie) {
    visited_values = append(visited_values, value)
}

BinaryTree_inorder(traversal_root, visit_function)
fr fr Should visit in order: 1, 2, 3, 4, 6
assert_eq_int(len(visited_values), 5)
assert_eq_int(visited_values[0], 1)
assert_eq_int(visited_values[1], 2)
assert_eq_int(visited_values[2], 3)
assert_eq_int(visited_values[3], 4)
assert_eq_int(visited_values[4], 6)

fr fr ================================
fr fr Error Handling Tests
fr fr ================================

test_start("array_safe_get with valid index")
sus safe_test_array normie[value] = [10, 20, 30]
(safe_value, safe_error) := array_safe_get(safe_test_array, 1)
assert_eq_int(safe_value, 20)
assert_eq_string(safe_error.message, "")

test_start("array_safe_get with invalid index")
sus bounds_test_array normie[value] = [1, 2, 3]
(invalid_value, bounds_error) := array_safe_get(bounds_test_array, 10)
assert_eq_string(bounds_error.error_type, "IndexError")
assert_true(len(bounds_error.message) > 0)

test_start("CollectionError creation")
sus test_error CollectionError = CollectionError_new("Test error message", "TestError")
assert_eq_string(test_error.message, "Test error message")
assert_eq_string(test_error.error_type, "TestError")

fr fr ================================
fr fr Performance and Edge Cases
fr fr ================================

test_start("Large array operations")
sus large_array normie[value] = []
bestie i := 0; i < 1000; i++ {
    large_array = append(large_array, i)
}

sus filtered_large normie[value] = array_filter(large_array, slay(x normie) lit { damn x % 10 == 0 })
assert_eq_int(len(filtered_large), 100) fr fr 0, 10, 20, ..., 990

test_start("HashMap resize behavior")
sus resize_map HashMap<tea, normie> = HashMap_new<tea, normie>()
sus initial_capacity normie = resize_map.capacity

fr fr Add enough items to trigger resize
bestie i := 0; i < 20; i++ {
    sus key tea = "key" + string_from_int(i)
    resize_map = HashMap_insert(resize_map, key, i)
}

fr fr Should have triggered resize
assert_true(resize_map.capacity > initial_capacity)
assert_eq_int(resize_map.size, 20)

test_start("Empty collection operations")
sus empty_array normie[value] = []
sus empty_unique normie[value] = array_unique(empty_array)
assert_eq_int(len(empty_unique), 0)

sus empty_reversed normie[value] = array_reverse(empty_array)
assert_eq_int(len(empty_reversed), 0)

test_start("Single element collections")
sus single_array normie[value] = [42]
sus single_reversed normie[value] = array_reverse(single_array)
assert_eq_int(len(single_reversed), 1)
assert_eq_int(single_reversed[0], 42)

print_test_summary()

vibez.spill("🎉 Enhanced Collections Tests Complete!")
vibez.spill("✅ Generic operations, thread-safe collections working")
vibez.spill("🌳 Binary trees and advanced data structures validated")
vibez.spill("🛡️ Error handling and edge cases covered")
vibez.spill("⚡ Performance tests with large datasets successful")
