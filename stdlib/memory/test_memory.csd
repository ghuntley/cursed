yeet "testz"
yeet "memory"

fr fr Comprehensive Memory Management Test Suite
test_start("Memory Management Operations Test")

fr fr Basic Memory Allocation Tests
sus addr1, err1 = memory_allocate(1024)
assert_not_null(string(addr1))
assert_eq_string(err1.(tea), "")
assert_gt(addr1, 0)

sus addr2, err2 = memory_allocate(2048)
assert_not_null(string(addr2))
assert_eq_string(err2.(tea), "")
assert_gt(addr2, 0)

fr fr Test different allocation addresses
assert_true(addr1 != addr2)

fr fr Memory Deallocation Tests
sus success1, dealloc_err1 = memory_deallocate(addr1)
assert_true(success1)
assert_eq_string(dealloc_err1.(tea), "")

sus success2, dealloc_err2 = memory_deallocate(addr2)
assert_true(success2)
assert_eq_string(dealloc_err2.(tea), "")

fr fr Error Condition Tests - Invalid Allocation Size
sus invalid_addr1, invalid_err1 = memory_allocate(0)
assert_eq_int(invalid_addr1, 0)
assert_not_null(invalid_err1.(tea))

sus invalid_addr2, invalid_err2 = memory_allocate(-100)
assert_eq_int(invalid_addr2, 0)
assert_not_null(invalid_err2.(tea))

fr fr Error Condition Tests - Null Pointer Deallocation
sus null_success, null_err = memory_deallocate(0)
assert_false(null_success)
assert_not_null(null_err.(tea))

fr fr Memory Reallocation Tests
sus realloc_addr, realloc_err1 = memory_allocate(512)
assert_gt(realloc_addr, 0)
assert_eq_string(realloc_err1.(tea), "")

sus new_addr, realloc_err2 = memory_reallocate(realloc_addr, 1024)
assert_gt(new_addr, 0)
assert_eq_string(realloc_err2.(tea), "")

fr fr Clean up reallocation test
sus cleanup_success, cleanup_err = memory_deallocate(new_addr)
assert_true(cleanup_success)

fr fr Memory Copy Tests
sus src_addr, src_err = memory_allocate(256)
assert_gt(src_addr, 0)
sus dest_addr, dest_err = memory_allocate(256)
assert_gt(dest_addr, 0)

sus copy_success, copy_err = memory_copy(dest_addr, src_addr, 128)
assert_true(copy_success)
assert_eq_string(copy_err.(tea), "")

fr fr Memory Move Tests
sus move_success, move_err = memory_move(dest_addr, src_addr, 64)
assert_true(move_success)
assert_eq_string(move_err.(tea), "")

fr fr Memory Set Tests
sus set_success, set_err = memory_set(dest_addr, 0, 256)
assert_true(set_success)
assert_eq_string(set_err.(tea), "")

fr fr Memory Compare Tests
sus compare_result, compare_err = memory_compare(src_addr, dest_addr, 64)
assert_eq_string(compare_err.(tea), "")

fr fr Clean up copy/move/set tests
memory_deallocate(src_addr)
memory_deallocate(dest_addr)

fr fr Memory Pool Tests
sus pool, pool_err = memory_pool_create(64, 10)
assert_eq_string(pool_err.(tea), "")

sus pool_addr1, pool_acquire_err1 = memory_pool_acquire(pool)
assert_gt(pool_addr1, 0)
assert_eq_string(pool_acquire_err1.(tea), "")

sus pool_addr2, pool_acquire_err2 = memory_pool_acquire(pool)
assert_gt(pool_addr2, 0)
assert_eq_string(pool_acquire_err2.(tea), "")

fr fr Pool addresses should be different
assert_true(pool_addr1 != pool_addr2)

fr fr Release pool blocks
sus pool_release1, pool_release_err1 = memory_pool_release(pool, pool_addr1)
assert_true(pool_release1)
assert_eq_string(pool_release_err1.(tea), "")

sus pool_release2, pool_release_err2 = memory_pool_release(pool, pool_addr2)
assert_true(pool_release2)
assert_eq_string(pool_release_err2.(tea), "")

fr fr Destroy pool
sus pool_destroy_success, pool_destroy_err = memory_pool_destroy(pool)
assert_true(pool_destroy_success)
assert_eq_string(pool_destroy_err.(tea), "")

fr fr Garbage Collection Tests
sus freed_bytes1, gc_err1 = memory_gc_collect()
assert_eq_string(gc_err1.(tea), "")
assert_gt(freed_bytes1, 0)

sus freed_bytes2, gc_err2 = memory_gc_force_collect()
assert_eq_string(gc_err2.(tea), "")
assert_gt(freed_bytes2, 0)

sus gc_stats = memory_gc_get_stats()
assert_not_null(string(gc_stats.total_allocated))

fr fr Memory Safety Tests
sus safety_addr, safety_err = memory_allocate(128)
assert_gt(safety_addr, 0)

sus bounds_check, bounds_err = memory_check_bounds(safety_addr, 128, 64)
assert_true(bounds_check)
assert_eq_string(bounds_err.(tea), "")

sus null_check, null_check_err = memory_check_null(safety_addr)
assert_true(null_check)
assert_eq_string(null_check_err.(tea), "")

sus double_free_check, double_free_err = memory_check_double_free(safety_addr)
assert_true(double_free_check)
assert_eq_string(double_free_err.(tea), "")

sus use_after_free_check, use_after_free_err = memory_check_use_after_free(safety_addr)
assert_true(use_after_free_check)
assert_eq_string(use_after_free_err.(tea), "")

fr fr Clean up safety tests
memory_deallocate(safety_addr)

fr fr Memory Statistics Tests
sus total_allocated = memory_get_total_allocated()
assert_gt(total_allocated, 0)

sus allocation_count = memory_get_allocation_count()
assert_gt(allocation_count, 0)

sus fragmentation_ratio = memory_get_fragmentation_ratio()
assert_true(fragmentation_ratio >= 0.0)

fr fr Test memory threshold setting
memory_gc_set_threshold(2097152) fr fr 2MB threshold

fr fr Error Condition Tests - Large Allocation
sus large_addr, large_err = memory_allocate(0x80000000) fr fr > 2GB
assert_eq_int(large_addr, 0)
assert_not_null(large_err.(tea))

fr fr Error Condition Tests - Null Pointer Operations
sus null_copy_success, null_copy_err = memory_copy(0, safety_addr, 64)
assert_false(null_copy_success)
assert_not_null(null_copy_err.(tea))

sus null_move_success, null_move_err = memory_move(0, safety_addr, 64)
assert_false(null_move_success)
assert_not_null(null_move_err.(tea))

sus null_set_success, null_set_err = memory_set(0, 0, 64)
assert_false(null_set_success)
assert_not_null(null_set_err.(tea))

sus null_compare_result, null_compare_err = memory_compare(0, safety_addr, 64)
assert_eq_int(null_compare_result, -2)
assert_not_null(null_compare_err.(tea))

fr fr Test reallocation edge cases
sus realloc_null_addr, realloc_null_err = memory_reallocate(0, 256)
assert_gt(realloc_null_addr, 0) fr fr Should work like malloc
assert_eq_string(realloc_null_err.(tea), "")

sus realloc_zero_addr, realloc_zero_err = memory_reallocate(realloc_null_addr, 0)
assert_eq_int(realloc_zero_addr, 0) fr fr Should work like free
assert_eq_string(realloc_zero_err.(tea), "")

fr fr Memory Pool Error Tests
sus invalid_pool, invalid_pool_err = memory_pool_create(0, 10)
assert_not_null(invalid_pool_err.(tea))

sus invalid_pool2, invalid_pool_err2 = memory_pool_create(64, 0)
assert_not_null(invalid_pool_err2.(tea))

vibez.spill("🧠 Memory management tests completed successfully!")

print_test_summary()
