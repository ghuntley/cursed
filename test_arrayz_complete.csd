yeet "testz"
yeet "arrayz"

test_start("Comprehensive Array Operations Test")

fr fr Test 1: array_push - Add element to end of array
sus test_array [tea] = arrayz.array_new()
test_array = arrayz.array_push(test_array, "first")
test_array = arrayz.array_push(test_array, "second")
test_array = arrayz.array_push(test_array, "third")
assert_eq_int(arrayz.array_length(test_array), 3)
assert_eq_string(arrayz.array_get(test_array, 0), "first")
assert_eq_string(arrayz.array_get(test_array, 2), "third")

fr fr Test 2: array_pop - Remove and return last element
sus popped_array, popped_value = arrayz.array_pop(test_array)
assert_eq_int(arrayz.array_length(popped_array), 2)
assert_eq_string(popped_value, "third")
assert_eq_string(arrayz.array_get(popped_array, 1), "second")

fr fr Test 3: array_sort - Sort array elements  
sus unsorted [tea] = ["zebra", "apple", "banana", "cherry"]
sus sorted [tea] = arrayz.array_sort(unsorted)
assert_eq_string(arrayz.array_get(sorted, 0), "apple")
assert_eq_string(arrayz.array_get(sorted, 1), "banana") 
assert_eq_string(arrayz.array_get(sorted, 2), "cherry")
assert_eq_string(arrayz.array_get(sorted, 3), "zebra")

fr fr Test 4: array_length - Get array length
sus length_test [tea] = ["a", "b", "c", "d", "e"]
assert_eq_int(arrayz.array_length(length_test), 5)
sus empty_array [tea] = arrayz.array_new()
assert_eq_int(arrayz.array_length(empty_array), 0)

fr fr Test 5: array_get - Get element at index
sus get_test [tea] = ["zero", "one", "two", "three"]
assert_eq_string(arrayz.array_get(get_test, 0), "zero")
assert_eq_string(arrayz.array_get(get_test, 2), "two")
assert_eq_string(arrayz.array_get(get_test, 3), "three")
fr fr Test bounds checking
assert_eq_string(arrayz.array_get(get_test, -1), "")
assert_eq_string(arrayz.array_get(get_test, 10), "")

fr fr Test 6: array_set - Set element at index
sus set_test [tea] = ["original1", "original2", "original3"]
sus modified [tea] = arrayz.array_set(set_test, 1, "modified")
assert_eq_string(arrayz.array_get(modified, 0), "original1")
assert_eq_string(arrayz.array_get(modified, 1), "modified")
assert_eq_string(arrayz.array_get(modified, 2), "original3")
fr fr Test bounds checking
sus unchanged [tea] = arrayz.array_set(set_test, -1, "invalid")
assert_true(arrayz.array_equals(unchanged, set_test))

test_start("Advanced Array Operations")

fr fr Test array_sort_numbers for numeric sorting
sus numbers [normie] = [5, 2, 8, 1, 9]
sus sorted_nums [normie] = arrayz.array_sort_numbers(numbers)
assert_eq_int(sorted_nums[0], 1)
assert_eq_int(sorted_nums[1], 2)
assert_eq_int(sorted_nums[2], 5)
assert_eq_int(sorted_nums[3], 8)
assert_eq_int(sorted_nums[4], 9)

fr fr Test array operations with empty arrays
sus empty [tea] = []
sus empty_popped, empty_value = arrayz.array_pop(empty)
assert_eq_int(arrayz.array_length(empty_popped), 0)
assert_eq_string(empty_value, "")

sus empty_sorted [tea] = arrayz.array_sort(empty)
assert_eq_int(arrayz.array_length(empty_sorted), 0)

fr fr Test array operations integration
sus integration_test [tea] = ["c", "a", "b"]
integration_test = arrayz.array_push(integration_test, "d")
integration_test = arrayz.array_sort(integration_test)
assert_eq_string(arrayz.array_get(integration_test, 0), "a")
assert_eq_string(arrayz.array_get(integration_test, 3), "d")

sus final_array, removed = arrayz.array_pop(integration_test)
assert_eq_string(removed, "d")
assert_eq_int(arrayz.array_length(final_array), 3)

print_test_summary()
