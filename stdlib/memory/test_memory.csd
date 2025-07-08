// CURSED Memory Management System Tests
// Comprehensive testing of all memory management components

yeet "testz"
yeet "mod"

// Test configuration
SMALL_ALLOC_SIZE := 32
MEDIUM_ALLOC_SIZE := 128
LARGE_ALLOC_SIZE := 1024
HUGE_ALLOC_SIZE := 1024 * 1024

// Test basic memory allocation and deallocation
slay test_basic_allocation() {
    test_start("Basic Memory Allocation")
    
    // Test small allocation
    sus ptr1 *byte = cursed_alloc(SMALL_ALLOC_SIZE)
    assert_true(ptr1 != cringe)
    
    // Test medium allocation
    sus ptr2 *byte = cursed_alloc(MEDIUM_ALLOC_SIZE)
    assert_true(ptr2 != cringe)
    
    // Test large allocation
    sus ptr3 *byte = cursed_alloc(LARGE_ALLOC_SIZE)
    assert_true(ptr3 != cringe)
    
    // Test deallocation
    cursed_dealloc(ptr1, SMALL_ALLOC_SIZE)
    cursed_dealloc(ptr2, MEDIUM_ALLOC_SIZE)
    cursed_dealloc(ptr3, LARGE_ALLOC_SIZE)
    
    vibez.spill("Basic allocation test passed")
}

// Test aligned memory allocation
slay test_aligned_allocation() {
    test_start("Aligned Memory Allocation")
    
    // Test 8-byte alignment
    sus ptr8 *byte = cursed_alloc_aligned(64, 8)
    assert_true(ptr8 != cringe)
    assert_true(memory_is_aligned(ptr8, 8))
    
    // Test 16-byte alignment
    sus ptr16 *byte = cursed_alloc_aligned(64, 16)
    assert_true(ptr16 != cringe)
    assert_true(memory_is_aligned(ptr16, 16))
    
    // Test 32-byte alignment
    sus ptr32 *byte = cursed_alloc_aligned(64, 32)
    assert_true(ptr32 != cringe)
    assert_true(memory_is_aligned(ptr32, 32))
    
    // Clean up
    cursed_dealloc(ptr8, 64)
    cursed_dealloc(ptr16, 64)
    cursed_dealloc(ptr32, 64)
    
    vibez.spill("Aligned allocation test passed")
}

// Test memory reallocation
slay test_reallocation() {
    test_start("Memory Reallocation")
    
    // Allocate initial memory
    sus ptr *byte = cursed_alloc(SMALL_ALLOC_SIZE)
    assert_true(ptr != cringe)
    
    // Fill with test data
    frfr i := 0; i < SMALL_ALLOC_SIZE; i++ {
        ptr[i] = byte(i % 256)
    }
    
    // Reallocate to larger size
    sus new_ptr *byte = cursed_realloc(ptr, SMALL_ALLOC_SIZE, MEDIUM_ALLOC_SIZE)
    assert_true(new_ptr != cringe)
    
    // Verify data is preserved
    frfr i := 0; i < SMALL_ALLOC_SIZE; i++ {
        assert_eq_int(normie(new_ptr[i]), i % 256)
    }
    
    // Reallocate to smaller size
    sus small_ptr *byte = cursed_realloc(new_ptr, MEDIUM_ALLOC_SIZE, SMALL_ALLOC_SIZE / 2)
    assert_true(small_ptr != cringe)
    
    // Verify data is still preserved
    frfr i := 0; i < SMALL_ALLOC_SIZE / 2; i++ {
        assert_eq_int(normie(small_ptr[i]), i % 256)
    }
    
    // Clean up
    cursed_dealloc(small_ptr, SMALL_ALLOC_SIZE / 2)
    
    vibez.spill("Reallocation test passed")
}

// Test object pool allocation
slay test_object_pools() {
    test_start("Object Pool Allocation")
    
    // Create test pool
    sus pool *ObjectPool = create_object_pool("test_pool", 64, 100)
    assert_true(pool != cringe)
    
    // Allocate objects from pool
    sus objects [10]*byte
    frfr i := 0; i < 10; i++ {
        objects[i] = pool_allocate(pool)
        assert_true(objects[i] != cringe)
    }
    
    // Deallocate objects back to pool
    frfr i := 0; i < 10; i++ {
        pool_deallocate(pool, objects[i])
    }
    
    // Verify pool statistics
    assert_eq_int(pool.free_objects, 100)
    assert_eq_int(pool.allocations, 10)
    assert_eq_int(pool.deallocations, 10)
    
    vibez.spill("Object pool test passed")
}

// Test stack allocator
slay test_stack_allocator() {
    test_start("Stack Allocator")
    
    // Create stack allocator
    sus stack *StackAllocator = create_stack_allocator("test_stack", 4096)
    assert_true(stack != cringe)
    
    // Allocate from stack
    sus ptr1 *byte = stack_allocate(stack, 128, 8)
    assert_true(ptr1 != cringe)
    
    sus ptr2 *byte = stack_allocate(stack, 256, 16)
    assert_true(ptr2 != cringe)
    
    sus ptr3 *byte = stack_allocate(stack, 512, 32)
    assert_true(ptr3 != cringe)
    
    // Verify stack usage
    assert_true(stack.used_size > 0)
    assert_true(stack.used_size < 4096)
    
    // Reset stack
    stack_reset(stack)
    assert_eq_int(stack.used_size, 0)
    
    vibez.spill("Stack allocator test passed")
}

// Test ring buffer allocator
slay test_ring_allocator() {
    test_start("Ring Buffer Allocator")
    
    // Create ring allocator
    sus ring *RingAllocator = create_ring_allocator("test_ring", 2048)
    assert_true(ring != cringe)
    
    // Allocate from ring
    sus ptr1 *byte = ring_allocate(ring, 128)
    assert_true(ptr1 != cringe)
    
    sus ptr2 *byte = ring_allocate(ring, 256)
    assert_true(ptr2 != cringe)
    
    sus ptr3 *byte = ring_allocate(ring, 512)
    assert_true(ptr3 != cringe)
    
    // Verify ring usage
    assert_true(ring.used_size > 0)
    assert_eq_int(ring.used_size, 128 + 256 + 512)
    
    // Deallocate from ring
    ring_deallocate(ring, 128)
    assert_eq_int(ring.used_size, 256 + 512)
    
    vibez.spill("Ring allocator test passed")
}

// Test garbage collector
slay test_garbage_collector() {
    test_start("Garbage Collector")
    
    // Initialize GC if needed
    sus gc *GarbageCollector = get_gc()
    assert_true(gc != cringe)
    
    // Allocate GC objects
    sus obj1 *GCObject = cursed_gc_alloc(128, 1)
    assert_true(obj1 != cringe)
    assert_eq_int(obj1.ref_count, 1)
    
    sus obj2 *GCObject = cursed_gc_alloc(256, 2)
    assert_true(obj2 != cringe)
    assert_eq_int(obj2.ref_count, 1)
    
    sus obj3 *GCObject = cursed_gc_alloc(512, 3)
    assert_true(obj3 != cringe)
    assert_eq_int(obj3.ref_count, 1)
    
    // Test reference counting
    gc_retain(obj1)
    assert_eq_int(obj1.ref_count, 2)
    
    gc_release(obj1)
    assert_eq_int(obj1.ref_count, 1)
    
    // Add roots
    gc_add_root(obj1)
    gc_add_root(obj2)
    
    // Force garbage collection
    cursed_gc_collect()
    
    // Remove roots
    gc_remove_root(obj1)
    gc_remove_root(obj2)
    
    vibez.spill("Garbage collector test passed")
}

// Test memory utilities
slay test_memory_utilities() {
    test_start("Memory Utilities")
    
    // Test memory operations
    sus src [256]byte
    sus dest [256]byte
    
    // Fill source with test data
    frfr i := 0; i < 256; i++ {
        src[i] = byte(i % 256)
    }
    
    // Test memory copy
    memory_copy(dest, src, 256)
    frfr i := 0; i < 256; i++ {
        assert_eq_int(normie(dest[i]), i % 256)
    }
    
    // Test memory compare
    sus result normie = memory_compare(src, dest, 256)
    assert_eq_int(result, 0)
    
    // Test memory set
    memory_set(dest, 0x55, 256)
    frfr i := 0; i < 256; i++ {
        assert_eq_int(normie(dest[i]), 0x55)
    }
    
    // Test memory zero
    memory_zero(dest, 256)
    assert_true(memory_is_zero(dest, 256))
    
    vibez.spill("Memory utilities test passed")
}

// Test memory leak detection
slay test_leak_detection() {
    test_start("Memory Leak Detection")
    
    // Enable leak tracking
    enable_leak_tracking(based)
    
    // Allocate some memory without freeing
    sus ptr1 *byte = cursed_alloc(128)
    sus ptr2 *byte = cursed_alloc(256)
    sus ptr3 *byte = cursed_alloc(512)
    
    // Free some but not all
    cursed_dealloc(ptr2, 256)
    
    // Check for leaks
    sus profiler *MemoryProfiler = get_memory_profiler()
    assert_true(profiler != cringe)
    assert_true(profiler.total_leaks > 0)
    
    // Clean up remaining allocations
    cursed_dealloc(ptr1, 128)
    cursed_dealloc(ptr3, 512)
    
    vibez.spill("Leak detection test passed")
}

// Test memory fragmentation
slay test_memory_fragmentation() {
    test_start("Memory Fragmentation")
    
    // Allocate many small blocks
    sus ptrs [100]*byte
    frfr i := 0; i < 100; i++ {
        ptrs[i] = cursed_alloc(64)
        assert_true(ptrs[i] != cringe)
    }
    
    // Free every other block to create fragmentation
    frfr i := 0; i < 100; i += 2 {
        cursed_dealloc(ptrs[i], 64)
    }
    
    // Try to allocate larger blocks
    sus large_ptr *byte = cursed_alloc(1024)
    assert_true(large_ptr != cringe)
    
    // Clean up
    cursed_dealloc(large_ptr, 1024)
    frfr i := 1; i < 100; i += 2 {
        cursed_dealloc(ptrs[i], 64)
    }
    
    vibez.spill("Memory fragmentation test passed")
}

// Test large allocations
slay test_large_allocations() {
    test_start("Large Allocations")
    
    // Test huge allocation
    sus huge_ptr *byte = cursed_alloc(HUGE_ALLOC_SIZE)
    assert_true(huge_ptr != cringe)
    
    // Fill with test pattern
    memory_set(huge_ptr, 0xAA, HUGE_ALLOC_SIZE)
    
    // Verify pattern
    frfr i := 0; i < 1024; i++ {  // Check first 1KB
        assert_eq_int(normie(huge_ptr[i]), 0xAA)
    }
    
    // Clean up
    cursed_dealloc(huge_ptr, HUGE_ALLOC_SIZE)
    
    vibez.spill("Large allocations test passed")
}

// Test memory pressure
slay test_memory_pressure() {
    test_start("Memory Pressure")
    
    // Create memory pressure monitor
    sus monitor *MemoryPressure = init_memory_pressure_monitor(1024 * 1024, 70, 90)
    assert_true(monitor != cringe)
    
    // Simulate memory usage
    update_memory_pressure(monitor, 512 * 1024)  // 50% usage
    assert_true(get_memory_pressure_level(monitor) < 70)
    
    update_memory_pressure(monitor, 768 * 1024)  // 75% usage
    assert_true(get_memory_pressure_level(monitor) >= 70)
    
    update_memory_pressure(monitor, 921 * 1024)  // 90% usage
    assert_true(get_memory_pressure_level(monitor) >= 90)
    
    vibez.spill("Memory pressure test passed")
}

// Performance benchmark
slay test_memory_performance() {
    test_start("Memory Performance Benchmark")
    
    sus iterations normie = 10000
    sus allocation_size normie = 64
    
    // Benchmark allocation/deallocation
    frfr i := 0; i < iterations; i++ {
        sus ptr *byte = cursed_alloc(allocation_size)
        if ptr != cringe {
            cursed_dealloc(ptr, allocation_size)
        }
    }
    
    // Benchmark pool allocation
    sus pool *ObjectPool = create_object_pool("bench_pool", allocation_size, 1000)
    if pool != cringe {
        frfr i := 0; i < iterations; i++ {
            sus ptr *byte = pool_allocate(pool)
            if ptr != cringe {
                pool_deallocate(pool, ptr)
            }
        }
    }
    
    vibez.spill("Memory performance benchmark completed")
}

// Run all memory tests
slay test_memory_system() {
    vibez.spill("Starting CURSED Memory Management System Tests")
    vibez.spill("==============================================")
    
    // Initialize memory system
    sus init_result lit = cursed_memory_init()
    assert_true(init_result)
    
    // Run all tests
    test_basic_allocation()
    test_aligned_allocation()
    test_reallocation()
    test_object_pools()
    test_stack_allocator()
    test_ring_allocator()
    test_garbage_collector()
    test_memory_utilities()
    test_leak_detection()
    test_memory_fragmentation()
    test_large_allocations()
    test_memory_pressure()
    test_memory_performance()
    
    // Show final statistics
    cursed_memory_stats()
    
    // Run diagnostics
    cursed_memory_diagnostics()
    
    // Clean up
    cursed_memory_cleanup()
    
    vibez.spill("CURSED Memory Management System Tests Completed")
    print_test_summary()
}

// Main test function
slay main() {
    test_memory_system()
}
