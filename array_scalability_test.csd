fr fr Array Scalability Test - Validate array operations beyond 5 elements
fr fr This test demonstrates the hardcoded limitations and validates fixes

yeet "arrayz"

fr fr Test data - arrays with 10, 50, and 100+ elements
sus small_array []drip = [1, 2, 3, 4, 5]
sus medium_array []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]
sus large_numbers []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
                           11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                           21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
                           31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
                           41, 42, 43, 44, 45, 46, 47, 48, 49, 50]

fr fr Test 1: Basic operations should work on large arrays
spill("=== Array Scalability Test ===")
spill("Small array (5 elements):", len(small_array))
spill("Medium array (20 elements):", len(medium_array)) 
spill("Large array (50 elements):", len(large_numbers))

fr fr Test 2: Sum operations (these should work fine)
sus small_sum drip = sum_array(small_array)
sus medium_sum drip = sum_array(medium_array)  
sus large_sum drip = sum_array(large_numbers)

spill("Small sum:", small_sum)
spill("Medium sum:", medium_sum)
spill("Large sum:", large_sum)

fr fr Test 3: Reverse operation (should fail for arrays > 5 elements)
spill("\n=== Reverse Operations ===")
sus small_reversed []drip = reverse_array(small_array)
sus medium_reversed []drip = reverse_array(medium_array)
sus large_reversed []drip = reverse_array(large_numbers)

spill("Small array reversed:", small_reversed)
spill("Medium array reversed (should be same as original due to limitation):", len(medium_reversed))
spill("Large array reversed (should be same as original due to limitation):", len(large_reversed))

fr fr Test 4: Sort operations (should fail for arrays > 3 elements)
spill("\n=== Sort Operations ===")
sus small_sorted []drip = sort_array_ascending([5, 3, 1, 4, 2])
sus medium_sorted []drip = sort_array_ascending(medium_array)
sus large_sorted []drip = sort_array_ascending(large_numbers)

spill("Small array (5 elements) sorted (should be original due to limitation):", small_sorted)
spill("Medium array sorted (should be original due to limitation):", len(medium_sorted))
spill("Large array sorted (should be original due to limitation):", len(large_sorted))

fr fr Test 5: Map operations (should fail for arrays > 3 elements)
spill("\n=== Map Operations ===")
sus small_doubled []drip = map_array(small_array, "double")
sus medium_doubled []drip = map_array(medium_array, "double") 
sus large_doubled []drip = map_array(large_numbers, "double")

spill("Small array doubled (should be original due to limitation):", len(small_doubled))
spill("Medium array doubled (should be original due to limitation):", len(medium_doubled))
spill("Large array doubled (should be original due to limitation):", len(large_doubled))

fr fr Test 6: Array concatenation (limited to small arrays)
spill("\n=== Concatenation Operations ===")
sus concat_small []drip = concat_arrays([1, 2], [3, 4])
sus concat_large []drip = concat_arrays(medium_array, large_numbers)

spill("Small concat result:", concat_small)
spill("Large concat result (should fallback to first array):", len(concat_large))

spill("\n=== Test Complete ===")
spill("This test demonstrates the hardcoded limitations in array operations.")
spill("Arrays beyond 3-5 elements return original arrays instead of processed results.")
