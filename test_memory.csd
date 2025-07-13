yeet "testz"
yeet "memory"

// Test heap allocation functions
test_start("malloc basic allocation")
sus ptr thicc = malloc(1024)
assert_true(ptr > 0)

test_start("malloc size tracking")
sus initial_allocated thicc = get_memory_usage()
sus new_ptr thicc = malloc(512)
sus final_allocated thicc = get_memory_usage()
assert_true(final_allocated > initial_allocated)

test_start("free function")
sus free_result lit = free(ptr)
assert_true(free_result)

test_start("realloc with valid pointer")
sus old_ptr thicc = malloc(256)
sus new_ptr thicc = realloc(old_ptr, 512)
assert_true(new_ptr > 0)

test_start("realloc with null pointer")
sus null_realloc thicc = realloc(0, 256)
assert_true(null_realloc > 0)

// Test garbage collection
test_start("gc_collect returns freed bytes")
sus freed_bytes normie = gc_collect()
assert_true(freed_bytes >= 0)

test_start("gc_stats returns string")
sus stats tea = gc_stats()
assert_true(stats.length > 0)

test_start("gc_pressure returns percentage")
sus pressure normie = gc_pressure()
assert_true(pressure >= 0)
assert_true(pressure <= 100)

// Test memory tracking
test_start("track_allocation with tag")
sus track_result lit = track_allocation(1024, "test_allocation")
assert_true(track_result)

test_start("memory_report returns string")
sus report tea = memory_report()
assert_true(report.length > 0)

// Test stack operations
test_start("get_stack_size returns positive value")
sus stack_size normie = get_stack_size()
assert_true(stack_size > 0)

test_start("check_stack_overflow returns boolean")
sus overflow_check lit = check_stack_overflow()
// Should return either based or cap
assert_true(overflow_check == based || overflow_check == cap)

// Test pool allocation
test_start("create_pool returns pool ID")
sus pool_id thicc = create_pool(64, 100)
assert_true(pool_id > 0)

test_start("pool_alloc from valid pool")
sus pool_ptr thicc = pool_alloc(pool_id, 64)
assert_true(pool_ptr > 0)

test_start("pool_free to valid pool")
sus pool_free_result lit = pool_free(pool_id, pool_ptr)
assert_true(pool_free_result)

test_start("pool_alloc from invalid pool")
sus invalid_pool_ptr thicc = pool_alloc(999, 64)
assert_true(invalid_pool_ptr == 0)

test_start("pool_free to invalid pool")
sus invalid_pool_free lit = pool_free(999, 123)
assert_false(invalid_pool_free)

// Test memory utility functions
test_start("zero_memory operation")
sus zero_result lit = zero_memory(123, 256)
assert_true(zero_result)

test_start("copy_memory operation")
sus copy_result lit = copy_memory(123, 456, 128)
assert_true(copy_result)

test_start("compare_memory operation")
sus compare_result normie = compare_memory(123, 123, 64)
assert_eq_int(compare_result, 0)

// Test memory alignment
test_start("align_size to 8-byte boundary")
sus aligned_size normie = align_size(100, 8)
assert_eq_int(aligned_size, 104)

test_start("align_size already aligned")
sus already_aligned normie = align_size(128, 8)
assert_eq_int(already_aligned, 128)

test_start("is_aligned check true")
sus aligned_check lit = is_aligned(128, 8)
assert_true(aligned_check)

test_start("is_aligned check false")
sus unaligned_check lit = is_aligned(129, 8)
assert_false(unaligned_check)

// Test advanced memory management
test_start("set_memory_limit")
sus limit_result lit = set_memory_limit(1048576)
assert_true(limit_result)

test_start("get_memory_usage returns current usage")
sus current_usage thicc = get_memory_usage()
assert_true(current_usage >= 0)

test_start("memory_compact returns compacted bytes")
sus compacted normie = memory_compact()
assert_true(compacted >= 0)

// Test multiple allocations and tracking
test_start("multiple allocations tracking")
sus ptr1 thicc = malloc(512)
sus ptr2 thicc = malloc(1024)
sus ptr3 thicc = malloc(256)

assert_true(ptr1 > 0)
assert_true(ptr2 > 0)
assert_true(ptr3 > 0)
assert_true(ptr1 != ptr2)
assert_true(ptr2 != ptr3)

// Test memory pressure under load
test_start("memory pressure calculation")
bestie i := 0; i < 10; i++ {
    sus temp_ptr thicc = malloc(1024)
}
sus high_pressure normie = gc_pressure()
assert_true(high_pressure >= 0)

// Test pool allocation efficiency
test_start("pool allocation efficiency")
sus efficient_pool thicc = create_pool(32, 50)
sus allocations_successful normie = 0

bestie j := 0; j < 10; j++ {
    sus temp_pool_ptr thicc = pool_alloc(efficient_pool, 32)
    if temp_pool_ptr > 0 {
        allocations_successful = allocations_successful + 1
    }
}
assert_true(allocations_successful > 0)

// Test memory statistics reset
test_start("reset_memory_stats")
sus reset_result lit = reset_memory_stats()
assert_true(reset_result)

sus usage_after_reset thicc = get_memory_usage()
assert_true(usage_after_reset == 0)

// Test edge cases
test_start("malloc with zero size")
sus zero_ptr thicc = malloc(0)
assert_true(zero_ptr >= 0)

test_start("align_size with size 1")
sus tiny_aligned normie = align_size(1, 4)
assert_eq_int(tiny_aligned, 4)

test_start("gc_collect multiple times")
sus gc1 normie = gc_collect()
sus gc2 normie = gc_collect()
sus gc3 normie = gc_collect()
assert_true(gc1 >= 0)
assert_true(gc2 >= 0)
assert_true(gc3 >= 0)

// Test memory report format
test_start("memory report contains expected data")
track_allocation(2048, "final_test")
sus final_report tea = memory_report()
assert_true(final_report.length > 0)

print_test_summary()
