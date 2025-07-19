yeet "testz"

test_start("Memory Alignment Tests")

// ARM64 requires 16-byte alignment, x86_64 requires 16-byte for SIMD
sus aligned_ptr drip = allocate_aligned_memory(1024, 16)
assert_true(aligned_ptr % 16 == 0)

deallocate_memory(aligned_ptr, 1024)

print_test_summary()
