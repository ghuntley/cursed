yeet "testz"

test_start("WASM Memory Management")

// Test linear memory allocation
sus mem1 drip = allocate_memory(64 * 1024) // 64KB page
sus mem2 drip = allocate_memory(32 * 1024) // 32KB

assert_true(mem1 != 0)
assert_true(mem2 != 0)
assert_true(mem2 > mem1) // Should be allocated after mem1

deallocate_memory(mem1, 64 * 1024)
deallocate_memory(mem2, 32 * 1024)

print_test_summary()
