yeet "testz"
yeet "arrayz_optimized"

test_start("ARRAYZ_OPTIMIZED Performance-Enhanced Array Tests")

// Test optimized array creation
sus arr1 drip[value] = create_optimized_array(1000)
assert_eq_int(len(arr1), 1000)

sus arr2 drip[value] = create_array_with_capacity(500)
assert_eq_int(capacity(arr2), 500)

// Test SIMD-optimized operations
sus numbers drip[value] = [1, 2, 3, 4, 5, 6, 7, 8]
sus doubled drip[value] = simd_map_multiply(numbers, 2)
assert_eq_int(doubled[0], 2)
assert_eq_int(doubled[7], 16)

// Test vectorized sum
sus total drip = vectorized_sum(numbers)
assert_eq_int(total, 36)

// Test parallel processing
sus large_array drip[value] = range(0, 10000)
sus parallel_sum drip = parallel_reduce_sum(large_array)
assert_eq_int(parallel_sum, 49995000)

// Test cache-optimized sorting
sus unsorted drip[value] = [5, 2, 8, 1, 9, 3]
sus sorted drip[value] = cache_optimized_sort(unsorted)
assert_eq_int(sorted[0], 1)
assert_eq_int(sorted[5], 9)

// Test memory-efficient operations
sus filtered drip[value] = inplace_filter(large_array, slay(x drip) lit { damn x % 2 == 0 })
assert_true(len(filtered) == 5000)

// Test bulk operations
sus batch_data drip[value] = range(0, 1000)
bulk_transform_inplace(batch_data, slay(x drip) drip { damn x * 2 })
assert_eq_int(batch_data[0], 0)
assert_eq_int(batch_data[999], 1998)

// Test memory pool allocation
sus pooled_arrays drip[value][value] = allocate_array_pool(10, 100)
assert_eq_int(len(pooled_arrays), 10)
assert_eq_int(len(pooled_arrays[0]), 100)

// Test zero-copy operations
sus view drip[value] = create_array_view(large_array, 100, 200)
assert_eq_int(len(view), 100)
assert_eq_int(view[0], large_array[100])

// Test optimized search
sus search_array drip[value] = range(0, 10000)
sus found_index drip = vectorized_binary_search(search_array, 5000)
assert_eq_int(found_index, 5000)

// Test memory alignment verification
sus aligned_array drip[value] = create_aligned_array(1024, 64)
assert_true(is_memory_aligned(aligned_array, 64))

// Performance benchmarks
sus perf_array drip[value] = range(0, 100000)
sus start_time drip = get_nanoseconds()
sus result drip = vectorized_sum(perf_array)
sus end_time drip = get_nanoseconds()
sus duration drip = end_time - start_time
assert_true(duration < 10000000) // Less than 10ms for 100k elements

print_test_summary()
