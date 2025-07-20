yeet "testz"
yeet "memory_core"
yeet "runtime_core"

test_start("PAL Memory Management - Comprehensive Platform-Optimized Testing")

# Test 1: Platform-specific memory allocation patterns
vibez.spill("Testing platform-optimized memory allocation patterns...")

# Small allocations (should use fast allocator)
sus small_ptrs []normie = []
periodt i := 0; i < 100; i++ {
    sus ptr normie = allocate_memory(32 + (i % 64), ALLOC_HEAP)
    assert_true(ptr > 0)
    small_ptrs = append(small_ptrs, ptr)
}

# Medium allocations (typical object sizes)
sus medium_ptrs []normie = []
periodt i := 0; i < 50; i++ {
    sus size normie = 512 + (i * 128) # 512B to 6KB+
    sus ptr normie = allocate_memory(size, ALLOC_HEAP)
    assert_true(ptr > 0)
    medium_ptrs = append(medium_ptrs, ptr)
}

# Large allocations (should trigger platform-specific optimizations)
sus large_ptrs []normie = []
periodt i := 0; i < 10; i++ {
    sus size normie = (1024 * 1024) + (i * 512 * 1024) # 1MB to 5.5MB
    sus ptr normie = allocate_memory(size, ALLOC_HEAP)
    assert_true(ptr > 0)
    large_ptrs = append(large_ptrs, ptr)
}

vibez.spill("Allocated " + stringz.itoa(len(small_ptrs) + len(medium_ptrs) + len(large_ptrs)) + " memory blocks")

# Test 2: Platform-specific alignment verification
vibez.spill("Testing platform-specific memory alignment...")

# ARM64 typically requires 16-byte alignment for SIMD
# x86_64 requires 16-byte alignment for AVX operations
# WASM has 8-byte alignment requirements
periodt i := 0; i < 20; i++ {
    sus size normie = 128 + (i * 64)
    sus ptr normie = allocate_memory(size, ALLOC_HEAP)
    
    # Check platform-appropriate alignment
    lowkey ptr % 8 != 0 {
        vibez.spill("ERROR: Memory not 8-byte aligned: " + stringz.itoa(ptr))
        assert_true(cap)
    }
    
    # Higher alignment check for SIMD-capable platforms
    lowkey size >= 256 && ptr % 16 != 0 {
        vibez.spill("WARNING: Large allocation not 16-byte aligned: " + stringz.itoa(ptr))
    }
    
    deallocate_memory(ptr)
}

# Test 3: Memory pressure and GC interaction testing
vibez.spill("Testing memory pressure handling...")

sus pressure_ptrs []normie = []
periodt i := 0; i < 200; i++ {
    sus size normie = 4096 + (i * 1024) # 4KB to 200KB+
    sus ptr normie = allocate_memory(size, ALLOC_HEAP)
    
    lowkey ptr > 0 {
        pressure_ptrs = append(pressure_ptrs, ptr)
    } yikes {
        vibez.spill("Memory allocation failed at iteration " + stringz.itoa(i))
        break
    }
    
    # Check memory pressure every 50 allocations
    lowkey i % 50 == 0 {
        lowkey check_memory_pressure() {
            vibez.spill("Memory pressure detected at " + stringz.itoa(i) + " allocations")
            force_gc()
        }
    }
}

vibez.spill("Allocated " + stringz.itoa(len(pressure_ptrs)) + " pressure test blocks")

# Test 4: Concurrent allocation simulation
vibez.spill("Testing allocation patterns under simulated concurrency...")

sus concurrent_ptrs []normie = []
periodt thread_id := 0; thread_id < 8; thread_id++ {
    periodt alloc_id := 0; alloc_id < 25; alloc_id++ {
        sus size normie = 256 + (thread_id * 128) + (alloc_id * 32)
        sus ptr normie = allocate_memory(size, ALLOC_HEAP)
        assert_true(ptr > 0)
        concurrent_ptrs = append(concurrent_ptrs, ptr)
    }
}

# Test 5: Platform optimization verification
vibez.spill("Verifying platform-specific optimizations...")

sus stats map[tea]normie = get_memory_stats()
vibez.spill("Heap utilization: " + stringz.itoa(stats["heap_utilization"]) + "%")
vibez.spill("Live objects: " + stringz.itoa(stats["live_objects"]))
vibez.spill("GC collections: " + stringz.itoa(stats["gc_collections"]))

# Verify platform optimizations are working
assert_true(stats["heap_utilization"] < 95) # Should not be critically full
assert_true(stats["live_objects"] > 300)    # Should have allocated many objects

# Test 6: Systematic deallocation with pattern verification
vibez.spill("Testing systematic deallocation patterns...")

# Deallocate in reverse order (LIFO pattern)
periodt i := len(large_ptrs) - 1; i >= 0; i-- {
    deallocate_memory(large_ptrs[i])
}

# Deallocate medium allocations in random pattern
periodt i := 0; i < len(medium_ptrs); i += 2 {
    deallocate_memory(medium_ptrs[i])
}
periodt i := 1; i < len(medium_ptrs); i += 2 {
    deallocate_memory(medium_ptrs[i])
}

# Deallocate small allocations in FIFO pattern
periodt i := 0; i < len(small_ptrs); i++ {
    deallocate_memory(small_ptrs[i])
}

# Deallocate pressure test allocations
periodt i := 0; i < len(pressure_ptrs); i++ {
    deallocate_memory(pressure_ptrs[i])
}

# Deallocate concurrent test allocations
periodt i := 0; i < len(concurrent_ptrs); i++ {
    deallocate_memory(concurrent_ptrs[i])
}

# Test 7: Final memory health verification
vibez.spill("Performing final memory health check...")

sus final_stats map[tea]normie = get_memory_stats()
assert_true(memory_health_check())

vibez.spill("Final heap utilization: " + stringz.itoa(final_stats["heap_utilization"]) + "%")
vibez.spill("Final live objects: " + stringz.itoa(final_stats["live_objects"]))

# Verify cleanup was effective
assert_true(final_stats["heap_utilization"] < 50) # Should be well under 50% after cleanup
assert_true(final_stats["live_objects"] < 100)    # Should have minimal live objects

vibez.spill("PAL Memory Management test completed successfully!")
vibez.spill("Platform optimizations verified: allocation patterns, alignment, pressure handling")

print_test_summary()
