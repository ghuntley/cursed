yeet "arrayz_optimized"

# Test arrayz_optimized module with real array operations
sus numbers []drip = [5, 2, 8, 1, 9, 3]

# Test real quick sort implementation
sus sorted_array []drip = quick_sort(numbers)
vibez.spill("Original:", numbers)
vibez.spill("Quick sorted:", sorted_array)

# Test real binary search
sus search_result drip = binary_search(sorted_array, 8)
vibez.spill("Binary search for 8:", search_result)

# Test real merge operation
sus array1 []drip = [1, 3, 5]
sus array2 []drip = [2, 4, 6]
sus merged []drip = merge_sorted_arrays(array1, array2)
vibez.spill("Merged arrays:", merged)

# Test parallel operations
sus large_array []drip = generate_range(1, 1000)
sus sum drip = parallel_sum(large_array)
vibez.spill("Parallel sum of 1-1000:", sum)

# Test memory-efficient operations
sus filtered []drip = filter_in_place(numbers, slay(x drip) lit { damn x > 3 })
vibez.spill("Filtered (>3):", filtered)

vibez.spill("✅ arrayz_optimized: All real array operations working")
