yeet "testz"

test_start("Memory Stress Test")

sus allocations drip = []

// Allocate memory in various sizes to test platform optimization
periodt i := 0; i < 1000; i++ {
    sus size drip = (i % 10 + 1) * 1024 // 1KB to 10KB
    sus ptr drip = allocate_memory(size)
    assert_true(ptr != 0)
    allocations.append([ptr, size])
}

// Deallocate in reverse order
periodt i := length(allocations) - 1; i >= 0; i-- {
    sus allocation drip = allocations[i]
    deallocate_memory(allocation[0], allocation[1])
}

print_test_summary()
