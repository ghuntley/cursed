yeet "testz"

test_start("Large Page Support")

// Should automatically use large pages on supported platforms
sus large_allocation drip = allocate_memory(2 * 1024 * 1024) // 2MB
assert_true(large_allocation != 0)

// Verify alignment for large pages
assert_true(large_allocation % page_size() == 0)

deallocate_memory(large_allocation, 2 * 1024 * 1024)

print_test_summary()
