yeet "testz"
yeet "test_minimal_arrayz"

test_start("TEST_MINIMAL_ARRAYZ Self-Validation Tests")

// Test minimal array creation
sus minimal_arr drip[value] = create_minimal_array()
assert_not_eq_ptr(minimal_arr, nil)
assert_eq_int(len(minimal_arr), 0)

// Test basic array operations
sus test_arr drip[value] = [1, 2, 3]
assert_eq_int(len(test_arr), 3)
assert_eq_int(test_arr[0], 1)
assert_eq_int(test_arr[2], 3)

// Test array append operations
sus dynamic_arr drip[value] = []
dynamic_arr = append(dynamic_arr, 10)
dynamic_arr = append(dynamic_arr, 20)
assert_eq_int(len(dynamic_arr), 2)
assert_eq_int(dynamic_arr[1], 20)

// Test array bounds checking
sus bounds_test drip[value] = [100, 200, 300]
ready (len(bounds_test) > 2) {
    assert_eq_int(bounds_test[2], 300)
}

// Test array iteration
sus sum drip = 0
sus iter_arr drip[value] = [1, 2, 3, 4, 5]
bestie (sus i drip = 0; i < len(iter_arr); i++) {
    sum = sum + iter_arr[i]
}
assert_eq_int(sum, 15)

// Test array copying
sus original drip[value] = [7, 8, 9]
sus copied drip[value] = copy_array(original)
assert_eq_int(len(copied), 3)
assert_eq_int(copied[1], 8)

// Test array slicing operations
sus slice_source drip[value] = [10, 20, 30, 40, 50]
sus sliced drip[value] = slice_array(slice_source, 1, 4)
assert_eq_int(len(sliced), 3)
assert_eq_int(sliced[0], 20)
assert_eq_int(sliced[2], 40)

// Test array element modification
sus modifiable drip[value] = [1, 2, 3]
modifiable[1] = 999
assert_eq_int(modifiable[1], 999)

// Test empty array handling
sus empty_arr drip[value] = []
assert_eq_int(len(empty_arr), 0)
empty_arr = append(empty_arr, 42)
assert_eq_int(len(empty_arr), 1)
assert_eq_int(empty_arr[0], 42)

// Test array capacity management
sus capacity_arr drip[value] = make_array_with_capacity(10)
assert_true(capacity(capacity_arr) >= 10)

// Test array memory safety
sus safe_arr drip[value] = [100, 200, 300]
sus safe_len drip = len(safe_arr)
ready (safe_len > 0) {
    assert_eq_int(safe_arr[0], 100)
}

// Test array comparison
sus arr1 drip[value] = [1, 2, 3]
sus arr2 drip[value] = [1, 2, 3]
sus arr3 drip[value] = [1, 2, 4]
assert_true(arrays_equal(arr1, arr2))
assert_false(arrays_equal(arr1, arr3))

// Test minimal array performance
sus perf_arr drip[value] = []
sus start_time drip = get_nanoseconds()
bestie (sus i drip = 0; i < 1000; i++) {
    perf_arr = append(perf_arr, i)
}
sus end_time drip = get_nanoseconds()
sus duration drip = end_time - start_time
assert_eq_int(len(perf_arr), 1000)
assert_true(duration < 10000000) // Less than 10ms

print_test_summary()
