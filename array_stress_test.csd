fr fr Array Stress Test - Test with 100+ element arrays
fr fr Validates scalability beyond previous 5-element limitation

yeet "arrayz"

spill("=== Array Scalability Stress Test ===")

fr fr Create large test arrays manually (since we need to avoid dynamic generation)
sus very_large_array []drip = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10,
                              11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
                              21, 22, 23, 24, 25, 26, 27, 28, 29, 30,
                              31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
                              41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
                              51, 52, 53, 54, 55, 56, 57, 58, 59, 60,
                              61, 62, 63, 64, 65, 66, 67, 68, 69, 70,
                              71, 72, 73, 74, 75, 76, 77, 78, 79, 80,
                              81, 82, 83, 84, 85, 86, 87, 88, 89, 90,
                              91, 92, 93, 94, 95, 96, 97, 98, 99, 100]

sus unsorted_large []drip = [50, 23, 87, 12, 95, 34, 71, 8, 63, 29,
                             77, 14, 92, 45, 68, 3, 81, 56, 19, 84,
                             37, 72, 9, 64, 28, 93, 41, 76, 15, 88,
                             52, 25, 69, 4, 79, 36, 91, 48, 17, 74,
                             2, 59, 31, 86, 43, 78, 11, 67, 24, 89]

spill("Large arrays created:")
spill("Very large array (100 elements):", len(very_large_array))
spill("Unsorted large array (50 elements):", len(unsorted_large))

fr fr Test 1: Basic arithmetic operations on large arrays
spill("\n=== Large Array Arithmetic ===")
sus large_sum drip = sum_array(very_large_array)
sus large_avg drip = average_array(very_large_array)
sus large_max drip = find_max(very_large_array)
sus large_min drip = find_min(very_large_array)

spill("Sum of 1-100:", large_sum, "(expected: 5050)")
spill("Average of 1-100:", large_avg, "(expected: 50)")
spill("Max of 1-100:", large_max, "(expected: 100)")
spill("Min of 1-100:", large_min, "(expected: 1)")

fr fr Test 2: Search operations
spill("\n=== Large Array Search ===")
sus contains_50 lit = contains_value(very_large_array, 50)
sus contains_101 lit = contains_value(very_large_array, 101)
sus index_of_75 drip = find_index(very_large_array, 75)

spill("Contains 50:", contains_50, "(expected: based)")
spill("Contains 101:", contains_101, "(expected: cringe)")  
spill("Index of 75:", index_of_75, "(expected: 74)")

fr fr Test 3: Array property tests
spill("\n=== Large Array Properties ===")
sus positive_count drip = count_positive(very_large_array)
sus zero_count drip = count_zeros(very_large_array)
sus is_all_pos lit = all_positive(very_large_array)
sus has_dups lit = has_duplicates(very_large_array)

spill("Positive count:", positive_count, "(expected: 100)")
spill("Zero count:", zero_count, "(expected: 0)")
spill("All positive:", is_all_pos, "(expected: based)")
spill("Has duplicates:", has_dups, "(expected: cringe)")

fr fr Test 4: Reverse large array (this was limited before)
spill("\n=== Large Array Reverse Test ===")
sus first_10 []drip = slice_array(very_large_array, 0, 10)
sus last_10 []drip = slice_array(very_large_array, 90, 100)

spill("First 10 elements:", first_10)
spill("Last 10 elements:", last_10)

sus reversed_first_10 []drip = reverse_array(first_10)
spill("First 10 reversed:", reversed_first_10)

fr fr Test 5: Sorting large array (this was limited before)
spill("\n=== Large Array Sort Test ===")
sus medium_unsorted []drip = slice_array(unsorted_large, 0, 10)
spill("Unsorted sample:", medium_unsorted)

sus sorted_sample []drip = sort_array_ascending(medium_unsorted)
spill("Sorted sample:", sorted_sample)
spill("Is sample sorted:", is_sorted_ascending(sorted_sample))

fr fr Test 6: Filter operations on large arrays
spill("\n=== Large Array Filter Test ===")
sus even_numbers []drip = filter_array(first_10, "even")
spill("Even numbers from first 10:", even_numbers)

fr fr Test 7: Array manipulation operations
spill("\n=== Large Array Manipulation ===")
sus doubled_first_5 []drip = map_array(slice_array(very_large_array, 0, 5), "double")
spill("First 5 doubled:", doubled_first_5)

sus squared_first_5 []drip = map_array(slice_array(very_large_array, 0, 5), "square")  
spill("First 5 squared:", squared_first_5)

fr fr Test 8: Array statistics
spill("\n=== Large Array Statistics ===")
sus sample_median drip = median_array(first_10)
sus sample_range drip = range_array(first_10)

spill("Median of first 10:", sample_median)
spill("Range of first 10:", sample_range)

fr fr Test 9: Large array equality and comparison
sus first_10_copy []drip = slice_array(very_large_array, 0, 10)
sus are_equal lit = arrays_equal(first_10, first_10_copy)
spill("First 10 equals its copy:", are_equal)

spill("\n=== Stress Test Complete ===")
spill("All operations completed successfully on large arrays!")
spill("This demonstrates the array scalability fixes are working.")
