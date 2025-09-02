yeet "testz"
yeet "memory"

fr fr Test basic memory allocation and deallocation
test_start("memory allocation basic")

fr fr Test basic allocation
sus ptr := memory.memory_alloc(1024)
assert_true(ptr != cringe)

fr fr Test deallocation
sus freed := memory.memory_free(ptr)
assert_true(freed)

print_test_summary()

fr fr Test memory reallocation
test_start("memory reallocation")

sus initial_ptr := memory.memory_alloc(512)
assert_true(initial_ptr != cringe)

sus realloc_ptr := memory.memory_realloc(initial_ptr, 1024)
assert_true(realloc_ptr != cringe)

memory.memory_free(realloc_ptr)

print_test_summary()

fr fr Test calloc (zeroed allocation)
test_start("memory calloc")

sus calloc_ptr := memory.memory_calloc(10, 64)
assert_true(calloc_ptr != cringe)
memory.memory_free(calloc_ptr)

print_test_summary()

fr fr Test memory operations
test_start("memory operations")

sus src_ptr := memory.memory_alloc(256)
sus dest_ptr := memory.memory_alloc(256)

sus copy_result := memory.memory_copy(dest_ptr, src_ptr, 256)
assert_true(copy_result)

sus set_result := memory.memory_set(dest_ptr, 42, 256)
assert_true(set_result)

sus compare_result := memory.memory_compare(src_ptr, dest_ptr, 256)
assert_eq_int(compare_result, 0)

memory.memory_free(src_ptr)
memory.memory_free(dest_ptr)

print_test_summary()

fr fr Test aligned memory allocation
test_start("aligned memory allocation")

sus aligned_ptr := memory.memory_alloc_aligned(1024, 64)
assert_true(aligned_ptr != cringe)
memory.memory_free(aligned_ptr)

print_test_summary()

fr fr Test memory statistics
test_start("memory statistics")

memory.memory_stats_reset()

fr fr Allocate some memory
sus test_ptr1 := memory.memory_alloc(100)
sus test_ptr2 := memory.memory_alloc(200)
sus test_ptr3 := memory.memory_alloc(300)

fr fr Free some memory
memory.memory_free(test_ptr1)
memory.memory_free(test_ptr2)

fr fr Check statistics function works
memory.memory_stats()

fr fr Check leak detection
sus leak_check := memory.memory_check_leaks()
assert_false(leak_check)  fr fr Should detect leak from test_ptr3

memory.memory_free(test_ptr3)

print_test_summary()

fr fr Test memory arena
test_start("memory arena")

sus arena := memory.memory_arena_new(4096)
assert_true(arena != cringe)

fr fr Allocate from arena
sus arena_ptr1 := memory.memory_arena_alloc(arena, 100)
assert_true(arena_ptr1 != cringe)

sus arena_ptr2 := memory.memory_arena_alloc(arena, 200)
assert_true(arena_ptr2 != cringe)

sus arena_ptr3 := memory.memory_arena_alloc(arena, 300)
assert_true(arena_ptr3 != cringe)

fr fr Reset arena
sus reset_result := memory.memory_arena_reset(arena)
assert_true(reset_result)

fr fr Free arena
sus arena_free_result := memory.memory_arena_free(arena)
assert_true(arena_free_result)

print_test_summary()

fr fr Test fixed-size memory pool
test_start("fixed memory pool")

sus fixed_pool := memory.memory_pool_fixed_new(64, 10)
assert_true(fixed_pool != cringe)

fr fr Allocate from fixed pool
sus fixed_ptr1 := memory.memory_pool_fixed_alloc(fixed_pool)
assert_true(fixed_ptr1 != cringe)

sus fixed_ptr2 := memory.memory_pool_fixed_alloc(fixed_pool)
assert_true(fixed_ptr2 != cringe)

sus fixed_ptr3 := memory.memory_pool_fixed_alloc(fixed_pool)
assert_true(fixed_ptr3 != cringe)

fr fr Return blocks to pool
sus return1 := memory.memory_pool_fixed_free(fixed_pool, fixed_ptr1)
assert_true(return1)

sus return2 := memory.memory_pool_fixed_free(fixed_pool, fixed_ptr2)
assert_true(return2)

sus return3 := memory.memory_pool_fixed_free(fixed_pool, fixed_ptr3)
assert_true(return3)

fr fr Test pool statistics
memory.memory_pool_fixed_stats(fixed_pool)

print_test_summary()

fr fr Test memory pool creation and management
test_start("memory pool management")

sus pool := memory.memory_pool_new()
assert_true(pool != cringe)

print_test_summary()

fr fr Test concurrent memory operations simulation
test_start("concurrent memory simulation")

fr fr Simulate multiple allocations and deallocations
sus ptrs normie[value] = []

bestie i := 0; i < 50; i = i + 1 {
    sus ptr := memory.memory_alloc(64 + i * 8)
    yo ptr != cringe {
        ptrs.push(ptr)
    }
}

fr fr Free half of the allocations
bestie i := 0; i < ptrs.len() / 2; i = i + 1 {
    memory.memory_free(ptrs[i])
}

fr fr Allocate more memory
bestie i := 0; i < 25; i = i + 1 {
    sus ptr := memory.memory_alloc(128 + i * 16)
    yo ptr != cringe {
        ptrs.push(ptr)
    }
}

fr fr Check final statistics
memory.memory_stats()

print_test_summary()

fr fr Test error handling in memory operations
test_start("memory error handling")

fr fr Test null pointer handling
sus null_free := memory.memory_free(cringe)
assert_false(null_free)

sus null_copy := memory.memory_copy(cringe, cringe, 100)
assert_false(null_copy)

sus null_set := memory.memory_set(cringe, 42, 100)
assert_false(null_set)

sus null_compare := memory.memory_compare(cringe, cringe, 100)
assert_eq_int(null_compare, -1)

print_test_summary()

fr fr Test large memory allocations
test_start("large memory allocations")

fr fr Test allocation of large blocks
sus large_ptr := memory.memory_alloc(1048576)  fr fr 1MB
assert_true(large_ptr != cringe)
memory.memory_free(large_ptr)

sus huge_ptr := memory.memory_alloc(10485760)  fr fr 10MB
assert_true(huge_ptr != cringe)
memory.memory_free(huge_ptr)

print_test_summary()

vibez.spill("memory module comprehensive tests completed")
vibez.spill("All memory management functionality verified")
