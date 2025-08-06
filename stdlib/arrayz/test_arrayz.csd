yeet "testz"
yeet "arrayz"

test_start("Array Creation Tests")

fr fr Test array_new
sus empty_array [tea] = arrayz.array_new()
assert_eq_int(arrayz.array_length(empty_array), 0)
assert_true(arrayz.array_is_empty(empty_array))

fr fr Test array_fill
sus filled_array [tea] = arrayz.array_fill(3, "test")
assert_eq_int(arrayz.array_length(filled_array), 3)
assert_eq_string(arrayz.array_get(filled_array, 0), "test")
assert_eq_string(arrayz.array_get(filled_array, 2), "test")

test_start("Basic Array Operations")

fr fr Test array_push and array_get
sus test_array [tea] = arrayz.array_new()
test_array = arrayz.array_push(test_array, "first")
test_array = arrayz.array_push(test_array, "second")
assert_eq_int(arrayz.array_length(test_array), 2)
assert_eq_string(arrayz.array_get(test_array, 0), "first")
assert_eq_string(arrayz.array_get(test_array, 1), "second")

fr fr Test array_set
test_array = arrayz.array_set(test_array, 0, "updated")
assert_eq_string(arrayz.array_get(test_array, 0), "updated")

test_start("Array Search Operations")

fr fr Test array_find and array_contains
sus search_array [tea] = ["apple", "banana", "cherry"]
assert_eq_int(arrayz.array_find(search_array, "banana"), 1)
assert_eq_int(arrayz.array_find(search_array, "orange"), -1)
assert_true(arrayz.array_contains(search_array, "apple"))
assert_false(arrayz.array_contains(search_array, "orange"))

test_start("Array Manipulation")

fr fr Test array_reverse
sus original [tea] = ["a", "b", "c"]
sus reversed [tea] = arrayz.array_reverse(original)
assert_eq_string(arrayz.array_get(reversed, 0), "c")
assert_eq_string(arrayz.array_get(reversed, 2), "a")

fr fr Test array_slice
sus slice_test [tea] = ["1", "2", "3", "4", "5"]
sus sliced [tea] = arrayz.array_slice(slice_test, 1, 4)
assert_eq_int(arrayz.array_length(sliced), 3)
assert_eq_string(arrayz.array_get(sliced, 0), "2")
assert_eq_string(arrayz.array_get(sliced, 2), "4")

fr fr Test array_concat
sus arr1 [tea] = ["a", "b"]
sus arr2 [tea] = ["c", "d"]
sus concatenated [tea] = arrayz.array_concat(arr1, arr2)
assert_eq_int(arrayz.array_length(concatenated), 4)
assert_eq_string(arrayz.array_get(concatenated, 2), "c")

test_start("Array Join and Split Operations")

fr fr Test array_join
sus join_test [tea] = ["hello", "world", "test"]
sus joined tea = arrayz.array_join(join_test, " ")
assert_eq_string(joined, "hello world test")

test_start("Array Comparison")

fr fr Test array_equals
sus comp1 [tea] = ["a", "b", "c"]
sus comp2 [tea] = ["a", "b", "c"]
sus comp3 [tea] = ["a", "b", "d"]
assert_true(arrayz.array_equals(comp1, comp2))
assert_false(arrayz.array_equals(comp1, comp3))

test_start("Array Utilities")

fr fr Test array_unique
sus with_duplicates [tea] = ["a", "b", "a", "c", "b"]
sus unique [tea] = arrayz.array_unique(with_duplicates)
assert_eq_int(arrayz.array_length(unique), 3)
assert_true(arrayz.array_contains(unique, "a"))
assert_true(arrayz.array_contains(unique, "b"))
assert_true(arrayz.array_contains(unique, "c"))

test_start("Numeric Array Operations")

fr fr Test number array operations
sus numbers [normie] = [3, 1, 4, 1, 5]
assert_eq_int(arrayz.array_sum_numbers(numbers), 14)
assert_eq_int(arrayz.array_min_numbers(numbers), 1)
assert_eq_int(arrayz.array_max_numbers(numbers), 5)

test_start("Array Validation")

fr fr Test array validation functions
sus all_digits [tea] = ["1", "2", "3"]
sus mixed [tea] = ["1", "a", "3"]

print_test_summary()
