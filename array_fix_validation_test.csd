fr fr Array Fix Validation Test - Test dynamic array operations
fr fr Validates that arrays work beyond hardcoded 5-element limits

yeet "arrayz"

spill("=== Array Fix Validation Test ===")

fr fr Test data
sus small_array []drip = [5, 3, 1, 4, 2]
sus medium_array []drip = [10, 6, 8, 3, 7, 1, 9, 4, 2, 5]  
sus large_array []drip = [15, 12, 18, 6, 9, 3, 11, 7, 14, 1, 13, 5, 16, 2, 10, 8, 17, 4, 19, 20]

spill("Test arrays created:")
spill("Small (5 elements):", len(small_array))
spill("Medium (10 elements):", len(medium_array))
spill("Large (20 elements):", len(large_array))

fr fr Test 1: Reverse operations on all sizes
spill("\n=== Reverse Operations Test ===")
sus small_reversed []drip = reverse_array(small_array)
sus medium_reversed []drip = reverse_array(medium_array)
sus large_reversed []drip = reverse_array(large_array)

spill("Small reversed:", small_reversed)
spill("Medium reversed length:", len(medium_reversed))
spill("Large reversed length:", len(large_reversed))

fr fr Test 2: Sorting operations
spill("\n=== Sorting Operations Test ===")
sus small_sorted []drip = sort_array_ascending(small_array)  
sus medium_sorted []drip = sort_array_ascending(medium_array)
sus large_sorted []drip = sort_array_ascending(large_array)

spill("Small sorted:", small_sorted)
spill("Medium sorted length:", len(medium_sorted), "first few:", medium_sorted[0], medium_sorted[1], medium_sorted[2])
spill("Large sorted length:", len(large_sorted), "first few:", large_sorted[0], large_sorted[1], large_sorted[2])

fr fr Test 3: Map operations (doubling)
spill("\n=== Map Operations Test ===")  
sus small_doubled []drip = map_array(small_array, "double")
sus medium_doubled []drip = map_array(medium_array, "double")
sus large_doubled []drip = map_array(large_array, "double")

spill("Small doubled:", small_doubled)
spill("Medium doubled length:", len(medium_doubled))
spill("Large doubled length:", len(large_doubled))

fr fr Test 4: Filter operations  
spill("\n=== Filter Operations Test ===")
sus small_evens []drip = filter_array(small_array, "even")
sus medium_evens []drip = filter_array(medium_array, "even")
sus large_evens []drip = filter_array(large_array, "even")

spill("Small evens:", small_evens)
spill("Medium evens length:", len(medium_evens))
spill("Large evens length:", len(large_evens))

fr fr Test 5: Array slicing
spill("\n=== Slicing Operations Test ===")
sus small_slice []drip = slice_array(small_array, 1, 3)
sus medium_slice []drip = slice_array(medium_array, 2, 6)
sus large_slice []drip = slice_array(large_array, 5, 10)

spill("Small slice [1:3]:", small_slice)
spill("Medium slice [2:6]:", medium_slice)  
spill("Large slice [5:10]:", large_slice)

fr fr Test 6: Array concatenation
spill("\n=== Concatenation Test ===")
sus concat_result []drip = concat_arrays(small_array, medium_array)
spill("Small + Medium length:", len(concat_result))

fr fr Test 7: Array insertion and removal
spill("\n=== Insertion/Removal Test ===")
sus inserted []drip = insert_at_index(small_array, 2, 99)
sus removed []drip = remove_at_index(inserted, 3)

spill("After inserting 99 at index 2:", inserted)
spill("After removing element at index 3:", removed)

fr fr Test 8: Array append operations
spill("\n=== Append Operations Test ===")
sus appended []drip = append_to_int_array(small_array, 42)
spill("Small array with 42 appended:", appended)

fr fr Test 9: Sorting verification
spill("\n=== Sorting Verification ===")
spill("Is small_sorted actually sorted?", is_sorted_ascending(small_sorted))
spill("Is medium_sorted actually sorted?", is_sorted_ascending(medium_sorted))
spill("Is large_sorted actually sorted?", is_sorted_ascending(large_sorted))

spill("\n=== Test Complete ===")
spill("If all operations show expected array lengths and results,")
spill("the dynamic array fixes are working correctly!")
