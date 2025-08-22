fr fr CURSED Memory Management Comprehensive Test Suite
fr fr P1 Critical - Tests for real memory operations and leak detection

yeet "memory/mod"
yeet "memory/profiler"
yeet "memory/gc_integration"
yeet "memory/bootstrap"
yeet "testz"
yeet "vibez"

slay run_comprehensive_memory_tests() {
    vibez.spill("CURSED Memory Management Test Suite")
    vibez.spill("=" * 50)
    
    fr fr Initialize memory systems
    bootstrap.bootstrap_init()
    
    fr fr Test 1: Basic memory operations
    test_basic_memory_operations()
    
    fr fr Test 2: Memory profiling and leak detection
    test_memory_profiling_and_leak_detection()
    
    fr fr Test 3: GC integration
    test_gc_integration()
    
    fr fr Test 4: Memory stress testing
    test_memory_stress_operations()
    
    fr fr Test 5: Arena and pool allocators
    test_arena_and_pool_allocators()
    
    fr fr Test 6: Memory utilities
    test_memory_utilities()
    
    fr fr Test 7: Error conditions
    test_memory_error_conditions()
    
    fr fr Final validation
    final_memory_validation()
    
    vibez.spill("\nMemory Management Test Suite Complete")
}

fr fr Test basic memory allocation, reallocation, and freeing
slay test_basic_memory_operations() {
    vibez.spill("\n--- Test 1: Basic Memory Operations ---")
    
    fr fr Test allocation
    sus ptr1 *void = mod.memory_alloc(1024)
    testz.assert_not_null(ptr1, "memory_alloc should return valid pointer")
    
    fr fr Test memory setting
    mod.memory_set(ptr1, 0xAA, 1024)
    sus byte_ptr *byte = ptr1
    testz.assert_eq_byte(byte_ptr[0], 0xAA, "memory_set should set first byte")
    testz.assert_eq_byte(byte_ptr[1023], 0xAA, "memory_set should set last byte")
    
    fr fr Test reallocation (grow)
    sus ptr2 *void = mod.memory_realloc(ptr1, 2048)
    testz.assert_not_null(ptr2, "memory_realloc should return valid pointer")
    
    fr fr Verify data preserved
    sus byte_ptr2 *byte = ptr2
    testz.assert_eq_byte(byte_ptr2[0], 0xAA, "realloc should preserve data")
    testz.assert_eq_byte(byte_ptr2[1023], 0xAA, "realloc should preserve data")
    
    fr fr Test calloc (zeroed allocation)
    sus ptr3 *void = mod.memory_calloc(256, 4)
    testz.assert_not_null(ptr3, "memory_calloc should return valid pointer")
    
    sus byte_ptr3 *byte = ptr3
    testz.assert_eq_byte(byte_ptr3[0], 0, "calloc should zero memory")
    testz.assert_eq_byte(byte_ptr3[1023], 0, "calloc should zero entire allocation")
    
    fr fr Test memory copying
    sus src_ptr *void = mod.memory_alloc(512)
    sus dest_ptr *void = mod.memory_alloc(512)
    
    mod.memory_set(src_ptr, 0xBB, 512)
    mod.memory_copy(dest_ptr, src_ptr, 512)
    
    sus dest_byte *byte = dest_ptr
    testz.assert_eq_byte(dest_byte[0], 0xBB, "memory_copy should copy first byte")
    testz.assert_eq_byte(dest_byte[511], 0xBB, "memory_copy should copy last byte")
    
    fr fr Test memory comparison
    sus result normie = mod.memory_compare(src_ptr, dest_ptr, 512)
    testz.assert_eq_int(result, 0, "memory_compare should return 0 for equal regions")
    
    fr fr Free all allocations
    mod.memory_free(ptr2)  fr fr ptr2 is the reallocated version of ptr1
    mod.memory_free(ptr3)
    mod.memory_free(src_ptr)
    mod.memory_free(dest_ptr)
    
    vibez.spill("✅ Basic memory operations test passed")
}

fr fr Test memory profiling and leak detection
slay test_memory_profiling_and_leak_detection() {
    vibez.spill("\n--- Test 2: Memory Profiling and Leak Detection ---")
    
    fr fr Enable profiler with full tracking
    profiler.profiler_enable(based, based, 10000)
    
    fr fr Clear previous statistics
    profiler.profiler_clear_stats()
    
    fr fr Allocate some memory
    sus test_ptrs []*void = []
    bestie i := 0; i < 10; i = i + 1 {
        sus ptr *void = mod.memory_alloc((i + 1) * 128)
        test_ptrs.push(ptr)
        profiler.profiler_track_allocation(ptr, (i + 1) * 128)
    }
    
    fr fr Free half of the allocations
    bestie i := 0; i < 5; i = i + 1 {
        mod.memory_free(test_ptrs[i])
        profiler.profiler_track_deallocation(test_ptrs[i])
    }
    
    fr fr Generate profiler report
    profiler.profiler_generate_report()
    
    fr fr Detect leaks (should find 5 leaks)
    sus has_leaks lit = profiler.profiler_detect_leaks()
    testz.assert_true(has_leaks, "profiler should detect memory leaks")
    
    fr fr Set leak threshold and test again
    profiler.profiler_set_leak_threshold(1024)  fr fr Only large leaks
    has_leaks = profiler.profiler_detect_leaks()
    testz.assert_true(has_leaks, "profiler should still detect large leaks")
    
    fr fr Clean up remaining allocations
    bestie i := 5; i < 10; i = i + 1 {
        mod.memory_free(test_ptrs[i])
        profiler.profiler_track_deallocation(test_ptrs[i])
    }
    
    fr fr Disable profiler
    profiler.profiler_disable()
    
    vibez.spill("✅ Memory profiling and leak detection test passed")
}

fr fr Test GC integration
slay test_gc_integration() {
    vibez.spill("\n--- Test 3: GC Integration ---")
    
    fr fr Configure GC
    gc_integration.gc_configure(cap, based, 512 * 1024)  fr fr 512KB threshold
    
    fr fr Allocate GC-managed objects
    sus gc_ptrs []*void = []
    bestie i := 0; i < 20; i = i + 1 {
        sus ptr *void = gc_integration.gc_allocate(256, 1, cringe)  fr fr Type 1 = array
        testz.assert_not_null(ptr, "gc_allocate should return valid pointer")
        gc_ptrs.push(ptr)
        
        fr fr Fill with test data
        mod.memory_set(ptr, (i + 1).(byte), 256)
    }
    
    fr fr Add some roots to prevent collection
    bestie i := 0; i < 10; i = i + 1 {
        gc_integration.gc_add_root(&gc_ptrs[i])
    }
    
    fr fr Force garbage collection
    gc_integration.gc_collect()
    
    fr fr Get GC statistics
    gc_integration.gc_get_stats()
    
    fr fr Verify rooted objects are still accessible
    bestie i := 0; i < 10; i = i + 1 {
        sus byte_ptr *byte = gc_ptrs[i]
        testz.assert_eq_byte(byte_ptr[0], (i + 1).(byte), "rooted objects should survive GC")
    }
    
    fr fr Remove roots
    bestie i := 0; i < 10; i = i + 1 {
        gc_integration.gc_remove_root(&gc_ptrs[i])
    }
    
    fr fr Shutdown GC
    gc_integration.gc_shutdown()
    
    vibez.spill("✅ GC integration test passed")
}

fr fr Test memory operations under stress
slay test_memory_stress_operations() {
    vibez.spill("\n--- Test 4: Memory Stress Testing ---")
    
    fr fr Stress test: many small allocations
    sus small_ptrs []*void = []
    bestie i := 0; i < 1000; i = i + 1 {
        sus ptr *void = mod.memory_alloc(32)
        testz.assert_not_null(ptr, "stress allocation should succeed")
        small_ptrs.push(ptr)
    }
    
    fr fr Free every other allocation
    bestie i := 0; i < 1000; i = i + 2 {
        mod.memory_free(small_ptrs[i])
    }
    
    fr fr Stress test: large allocations
    sus large_ptrs []*void = []
    bestie i := 0; i < 10; i = i + 1 {
        sus size normie = (i + 1) * 1024 * 1024  fr fr 1MB to 10MB
        sus ptr *void = mod.memory_alloc(size)
        yo ptr != cringe {
            large_ptrs.push(ptr)
            fr fr Fill with pattern
            mod.memory_set(ptr, (i + 0x10).(byte), size)
        }
    }
    
    fr fr Verify large allocation integrity
    bestie i := 0; i < large_ptrs.len(); i = i + 1 {
        sus ptr *byte = large_ptrs[i]
        sus expected byte = (i + 0x10).(byte)
        testz.assert_eq_byte(ptr[0], expected, "large allocation should preserve data")
        
        sus size normie = (i + 1) * 1024 * 1024
        testz.assert_eq_byte(ptr[size - 1], expected, "large allocation end should preserve data")
    }
    
    fr fr Clean up stress test allocations
    bestie i := 1; i < 1000; i = i + 2 {  fr fr Free remaining small allocations
        mod.memory_free(small_ptrs[i])
    }
    
    bestie i := 0; i < large_ptrs.len(); i = i + 1 {
        mod.memory_free(large_ptrs[i])
    }
    
    vibez.spill("✅ Memory stress testing passed")
}

fr fr Test arena and pool allocators
slay test_arena_and_pool_allocators() {
    vibez.spill("\n--- Test 5: Arena and Pool Allocators ---")
    
    fr fr Test memory arena
    sus arena *mod.MemoryArena = mod.memory_arena_new(64 * 1024)  fr fr 64KB arena
    testz.assert_not_null(arena, "memory_arena_new should succeed")
    
    fr fr Allocate from arena
    sus arena_ptrs []*void = []
    bestie i := 0; i < 100; i = i + 1 {
        sus ptr *void = mod.memory_arena_alloc(arena, 512)
        yo ptr != cringe {
            arena_ptrs.push(ptr)
            mod.memory_set(ptr, (i % 256).(byte), 512)
        } otherwise {
            break  fr fr Arena exhausted
        }
    }
    
    testz.assert_true(arena_ptrs.len() > 50, "arena should handle many allocations")
    
    fr fr Verify arena allocation integrity
    bestie i := 0; i < arena_ptrs.len(); i = i + 1 {
        sus ptr *byte = arena_ptrs[i]
        sus expected byte = (i % 256).(byte)
        testz.assert_eq_byte(ptr[0], expected, "arena allocation should preserve data")
    }
    
    fr fr Reset arena (frees all at once)
    mod.memory_arena_reset(arena)
    
    fr fr Test arena reuse after reset
    sus ptr *void = mod.memory_arena_alloc(arena, 1024)
    testz.assert_not_null(ptr, "arena should be reusable after reset")
    
    fr fr Free arena
    mod.memory_arena_free(arena)
    
    fr fr Test fixed pool
    sus pool *mod.FixedPool = mod.memory_pool_fixed_new(256, 20)  fr fr 256-byte blocks, 20 initial
    testz.assert_not_null(pool, "memory_pool_fixed_new should succeed")
    
    fr fr Allocate from pool
    sus pool_ptrs []*void = []
    bestie i := 0; i < 30; i = i + 1 {  fr fr More than initial count
        sus ptr *void = mod.memory_pool_fixed_alloc(pool)
        testz.assert_not_null(ptr, "pool allocation should succeed")
        pool_ptrs.push(ptr)
        mod.memory_set(ptr, (i + 0x20).(byte), 256)
    }
    
    fr fr Return some blocks to pool
    bestie i := 0; i < 15; i = i + 1 {
        mod.memory_pool_fixed_free(pool, pool_ptrs[i])
    }
    
    fr fr Get pool statistics
    mod.memory_pool_fixed_stats(pool)
    
    fr fr Allocate again (should reuse freed blocks)
    bestie i := 0; i < 10; i = i + 1 {
        sus ptr *void = mod.memory_pool_fixed_alloc(pool)
        testz.assert_not_null(ptr, "pool should reuse freed blocks")
    }
    
    vibez.spill("✅ Arena and pool allocators test passed")
}

fr fr Test memory utility functions
slay test_memory_utilities() {
    vibez.spill("\n--- Test 6: Memory Utilities ---")
    
    fr fr Test aligned allocation
    sus aligned_ptr *void = mod.memory_alloc_aligned(1024, 64)  fr fr 64-byte alignment
    testz.assert_not_null(aligned_ptr, "aligned allocation should succeed")
    
    fr fr Verify alignment
    sus ptr_int normie = aligned_ptr.(normie)
    testz.assert_eq_int(ptr_int % 64, 0, "pointer should be 64-byte aligned")
    
    fr fr Test overlapping memory copy
    sus overlap_src *void = mod.memory_alloc(1024)
    mod.memory_set(overlap_src, 0xCC, 1024)
    
    fr fr Copy with overlap (dest after src)
    sus overlap_dest *void = (*byte)(overlap_src) + 512
    mod.memory_copy(overlap_dest, overlap_src, 256)
    
    sus dest_bytes *byte = overlap_dest
    testz.assert_eq_byte(dest_bytes[0], 0xCC, "overlapping copy should work")
    testz.assert_eq_byte(dest_bytes[255], 0xCC, "overlapping copy should work")
    
    fr fr Test memory comparison edge cases
    sus cmp1 *void = mod.memory_alloc(256)
    sus cmp2 *void = mod.memory_alloc(256)
    
    mod.memory_set(cmp1, 0x11, 256)
    mod.memory_set(cmp2, 0x11, 256)
    
    sus cmp_result normie = mod.memory_compare(cmp1, cmp2, 256)
    testz.assert_eq_int(cmp_result, 0, "identical regions should compare equal")
    
    fr fr Make them different
    sus cmp1_bytes *byte = cmp1
    cmp1_bytes[128] = 0x22
    
    cmp_result = mod.memory_compare(cmp1, cmp2, 256)
    testz.assert_true(cmp_result != 0, "different regions should not compare equal")
    
    fr fr Clean up
    mod.memory_free(aligned_ptr)
    mod.memory_free(overlap_src)
    mod.memory_free(cmp1)
    mod.memory_free(cmp2)
    
    vibez.spill("✅ Memory utilities test passed")
}

fr fr Test error conditions and edge cases
slay test_memory_error_conditions() {
    vibez.spill("\n--- Test 7: Error Conditions ---")
    
    fr fr Test null pointer handling
    sus result lit = mod.memory_free(cringe)  fr fr Should handle null gracefully
    testz.assert_true(result, "memory_free should handle null pointer")
    
    fr fr Test zero-size allocation
    sus zero_ptr *void = mod.memory_alloc(0)
    testz.assert_null(zero_ptr, "zero-size allocation should return null")
    
    fr fr Test invalid realloc
    sus invalid_realloc *void = mod.memory_realloc(cringe, 1024)
    testz.assert_not_null(invalid_realloc, "realloc with null should allocate new")
    mod.memory_free(invalid_realloc)
    
    fr fr Test calloc overflow
    sus overflow_ptr *void = mod.memory_calloc(0xFFFFFFFF, 2)  fr fr Should overflow
    testz.assert_null(overflow_ptr, "calloc overflow should return null")
    
    fr fr Test memory operations on null
    sus null_result lit = mod.memory_copy(cringe, cringe, 100)
    testz.assert_false(null_result, "copy with null pointers should fail")
    
    null_result = mod.memory_set(cringe, 0, 100)
    testz.assert_false(null_result, "set with null pointer should fail")
    
    sus null_cmp_result normie = mod.memory_compare(cringe, cringe, 100)
    testz.assert_eq_int(null_cmp_result, -1, "compare with null should return -1")
    
    fr fr Test invalid alignment
    sus bad_align_ptr *void = mod.memory_alloc_aligned(1024, 3)  fr fr Not power of 2
    testz.assert_null(bad_align_ptr, "invalid alignment should return null")
    
    bad_align_ptr = mod.memory_alloc_aligned(1024, 0)  fr fr Zero alignment
    testz.assert_null(bad_align_ptr, "zero alignment should return null")
    
    vibez.spill("✅ Error conditions test passed")
}

fr fr Final memory validation and cleanup
slay final_memory_validation() {
    vibez.spill("\n--- Final Memory Validation ---")
    
    fr fr Get final memory statistics
    mod.memory_stats()
    
    fr fr Check for memory leaks
    sus leak_status lit = mod.memory_check_leaks()
    
    yo leak_status {
        vibez.spill("⚠️  Memory leaks detected!")
    } otherwise {
        vibez.spill("✅ No memory leaks detected")
    }
    
    fr fr Reset memory statistics
    mod.memory_stats_reset()
    
    fr fr Bootstrap validation
    bootstrap.bootstrap_validate()
    bootstrap.bootstrap_get_stats()
    bootstrap.bootstrap_cleanup()
    
    vibez.spill("✅ Final memory validation complete")
}

fr fr Main test runner
slay main() {
    testz.test_start("CURSED Memory Management Comprehensive Test")
    
    run_comprehensive_memory_tests()
    
    testz.print_test_summary()
}
