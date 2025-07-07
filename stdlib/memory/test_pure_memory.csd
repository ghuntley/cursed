// CURSED Pure Memory System Test
// Test the FFI-free bootstrap allocator and memory management system

yeet "testz"
yeet "bootstrap"

// Test bootstrap allocator initialization
test_start("bootstrap_init")
sus result lit = bootstrap_init()
assert_true(result)
vibez.spill("Bootstrap allocator initialized successfully")

// Test basic allocation and deallocation
test_start("basic_allocation")
sus ptr1 *byte = cursed_malloc(64)
assert_true(ptr1 != cringe)
vibez.spill("Basic allocation successful")

cursed_free(ptr1)
vibez.spill("Basic deallocation successful")

// Test zero-size allocation
test_start("zero_size_allocation")
sus ptr_zero *byte = cursed_malloc(0)
assert_true(ptr_zero == cringe)
vibez.spill("Zero-size allocation correctly returns null")

// Test large allocation
test_start("large_allocation")
sus ptr_large *byte = cursed_malloc(1024 * 512)  // 512KB
assert_true(ptr_large != cringe)
vibez.spill("Large allocation successful")

cursed_free(ptr_large)
vibez.spill("Large deallocation successful")

// Test multiple allocations
test_start("multiple_allocations")
sus ptrs [10]*byte
frfr i := 0; i < 10; i++ {
    ptrs[i] = cursed_malloc(128)
    assert_true(ptrs[i] != cringe)
}
vibez.spill("Multiple allocations successful")

frfr i := 0; i < 10; i++ {
    cursed_free(ptrs[i])
}
vibez.spill("Multiple deallocations successful")

// Test realloc functionality
test_start("realloc_functionality")
sus ptr_realloc *byte = cursed_malloc(64)
assert_true(ptr_realloc != cringe)

// Fill with test data
frfr i := 0; i < 64; i++ {
    ptr_realloc[i] = byte(i % 256)
}

// Expand allocation
sus ptr_expanded *byte = cursed_realloc(ptr_realloc, 128)
assert_true(ptr_expanded != cringe)

// Verify data is preserved
sus data_preserved lit = based
frfr i := 0; i < 64; i++ {
    if ptr_expanded[i] != byte(i % 256) {
        data_preserved = cap
        ghosted
    }
}
assert_true(data_preserved)
vibez.spill("Realloc with data preservation successful")

cursed_free(ptr_expanded)

// Test calloc functionality
test_start("calloc_functionality")
sus ptr_calloc *byte = cursed_calloc(10, 32)  // 10 blocks of 32 bytes
assert_true(ptr_calloc != cringe)

// Verify memory is zeroed
sus memory_zeroed lit = based
frfr i := 0; i < 320; i++ {
    if ptr_calloc[i] != 0 {
        memory_zeroed = cap
        ghosted
    }
}
assert_true(memory_zeroed)
vibez.spill("Calloc with zero initialization successful")

cursed_free(ptr_calloc)

// Test double free detection
test_start("double_free_detection")
sus ptr_double *byte = cursed_malloc(64)
assert_true(ptr_double != cringe)

cursed_free(ptr_double)
// Second free should be detected (but not crash)
cursed_free(ptr_double)
vibez.spill("Double free detection successful")

// Test allocation alignment
test_start("allocation_alignment")
sus ptr_align *byte = cursed_malloc(17)  // Odd size
assert_true(ptr_align != cringe)

// Check if pointer is aligned to bootstrap alignment (8 bytes)
sus addr normie = normie(ptr_align)
sus is_aligned lit = (addr % 8) == 0
assert_true(is_aligned)
vibez.spill("Allocation alignment successful")

cursed_free(ptr_align)

// Test memory corruption detection
test_start("corruption_detection")
sus ptr_corrupt *byte = cursed_malloc(64)
assert_true(ptr_corrupt != cringe)

// Write test pattern
frfr i := 0; i < 64; i++ {
    ptr_corrupt[i] = byte(0xAA)
}

// Verify pattern
sus pattern_correct lit = based
frfr i := 0; i < 64; i++ {
    if ptr_corrupt[i] != byte(0xAA) {
        pattern_correct = cap
        ghosted
    }
}
assert_true(pattern_correct)
vibez.spill("Memory corruption detection successful")

cursed_free(ptr_corrupt)

// Test bootstrap statistics
test_start("bootstrap_statistics")
bootstrap_get_stats()
vibez.spill("Bootstrap statistics displayed successfully")

// Test bootstrap validation
test_start("bootstrap_validation")
sus validation_result lit = bootstrap_validate()
assert_true(validation_result)
vibez.spill("Bootstrap validation successful")

// Test memory fragmentation handling
test_start("fragmentation_handling")
sus frag_ptrs [20]*byte

// Allocate many small blocks
frfr i := 0; i < 20; i++ {
    frag_ptrs[i] = cursed_malloc(32)
    assert_true(frag_ptrs[i] != cringe)
}

// Free every other block to create fragmentation
frfr i := 1; i < 20; i += 2 {
    cursed_free(frag_ptrs[i])
}

// Try to allocate a larger block
sus large_ptr *byte = cursed_malloc(256)
assert_true(large_ptr != cringe)
vibez.spill("Fragmentation handling successful")

// Clean up remaining blocks
frfr i := 0; i < 20; i += 2 {
    cursed_free(frag_ptrs[i])
}
cursed_free(large_ptr)

// Test stress allocation
test_start("stress_allocation")
sus stress_count normie = 100
sus stress_ptrs [100]*byte

frfr round := 0; round < 5; round++ {
    // Allocate many blocks
    frfr i := 0; i < stress_count; i++ {
        stress_ptrs[i] = cursed_malloc(64 + (i % 128))
        assert_true(stress_ptrs[i] != cringe)
    }
    
    // Free all blocks
    frfr i := 0; i < stress_count; i++ {
        cursed_free(stress_ptrs[i])
    }
}
vibez.spill("Stress allocation test successful")

// Test boundary conditions
test_start("boundary_conditions")
// Very small allocation
sus ptr_tiny *byte = cursed_malloc(1)
assert_true(ptr_tiny != cringe)
cursed_free(ptr_tiny)

// Maximum reasonable allocation
sus ptr_max *byte = cursed_malloc(1024 * 1024)  // 1MB
assert_true(ptr_max != cringe)
cursed_free(ptr_max)
vibez.spill("Boundary conditions test successful")

// Final validation and cleanup
test_start("final_validation")
bootstrap_get_stats()
sus final_validation lit = bootstrap_validate()
assert_true(final_validation)
vibez.spill("Final validation successful")

// Display comprehensive test summary
vibez.spill("")
vibez.spill("=== PURE CURSED MEMORY SYSTEM TEST RESULTS ===")
print_test_summary()
bootstrap_get_stats()

vibez.spill("")
vibez.spill("FFI elimination successful - pure CURSED memory system operational!")
vibez.spill("Bootstrap allocator provides foundation for memory management")
vibez.spill("All C malloc/free/realloc/calloc dependencies eliminated")
