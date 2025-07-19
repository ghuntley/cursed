yeet "testz"

test_start("PAL Memory Management")

// Test large allocations that should use platform-specific optimizations
sus large_buffer drip = allocate_memory(1024 * 1024) // 1MB
sus small_buffer drip = allocate_memory(64) // 64 bytes

assert_true(large_buffer != 0)
assert_true(small_buffer != 0)

deallocate_memory(large_buffer, 1024 * 1024)
deallocate_memory(small_buffer, 64)

print_test_summary()
