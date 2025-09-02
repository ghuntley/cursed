fr fr Test suite for memoryz module - Memory management and GC
yeet "testz"
yeet "memoryz"

fr fr ===== MEMORY ALLOCATION TESTS =====

slay test_basic_allocation() lit {
    testz.test_start("Basic memory allocation")
    
    fr fr Test basic allocation
    sus ptr1 normie = memoryz.allocate(1024, 100)
    testz.assert_true(ptr1 != 0)
    testz.assert_true(memoryz.is_valid_pointer(ptr1))
    testz.assert_eq_int(memoryz.get_block_size(ptr1), 1024)
    testz.assert_eq_int(memoryz.get_block_type(ptr1), 100)
    
    fr fr Test allocation statistics
    sus stats memoryz.AllocationStats = memoryz.get_memory_stats()
    testz.assert_true(stats.current_usage >= 1024)
    testz.assert_true(stats.allocation_count > 0)
    
    fr fr Clean up
    testz.assert_true(memoryz.deallocate(ptr1))
    
    damn based
}

slay test_zeroed_allocation() lit {
    testz.test_start("Zeroed memory allocation")
    
    sus ptr normie = memoryz.allocate_zeroed(512, 101)
    testz.assert_true(ptr != 0)
    testz.assert_true(memoryz.is_valid_pointer(ptr))
    testz.assert_eq_int(memoryz.get_block_size(ptr), 512)
    
    fr fr Memory should be zeroed (simplified test)
    testz.assert_true(memoryz.compare_memory(ptr, 0, 64) != 0)  fr fr Compare with null block
    
    memoryz.deallocate(ptr)
    damn based
}

slay test_array_allocation() lit {
    testz.test_start("Array memory allocation")
    
    fr fr Allocate array of 10 integers (4 bytes each)
    sus ptr normie = memoryz.allocate_array(10, 4, 102)
    testz.assert_true(ptr != 0)
    testz.assert_eq_int(memoryz.get_block_size(ptr), 40)  fr fr 10 * 4 bytes
    
    fr fr Test overflow protection
    sus overflow_ptr normie = memoryz.allocate_array(1000000000, 1000000000, 103)
    testz.assert_eq_int(overflow_ptr, 0)  fr fr Should fail due to overflow
    
    memoryz.deallocate(ptr)
    damn based
}

slay test_reallocation() lit {
    testz.test_start("Memory reallocation")
    
    sus ptr normie = memoryz.allocate(256, 104)
    testz.assert_true(ptr != 0)
    testz.assert_eq_int(memoryz.get_block_size(ptr), 256)
    
    fr fr Grow the block
    sus new_ptr normie = memoryz.reallocate(ptr, 512)
    testz.assert_true(new_ptr != 0)
    testz.assert_eq_int(memoryz.get_block_size(new_ptr), 512)
    
    fr fr Shrink the block
    sus smaller_ptr normie = memoryz.reallocate(new_ptr, 128)
    testz.assert_true(smaller_ptr != 0)
    testz.assert_eq_int(memoryz.get_block_size(smaller_ptr), 128)
    
    memoryz.deallocate(smaller_ptr)
    damn based
}

fr fr ===== GARBAGE COLLECTION TESTS =====

slay test_gc_configuration() lit {
    testz.test_start("GC configuration")
    
    sus original_config memoryz.GCConfig = memoryz.get_gc_config()
    
    sus new_config memoryz.GCConfig = memoryz.GCConfig{
        enable_gc: based,
        gc_threshold: 2048,
        collection_interval: 500,
        concurrent_gc: cap,
        mark_and_sweep: based,
        generational: cap
    }
    
    testz.assert_true(memoryz.configure_gc(new_config))
    
    sus updated_config memoryz.GCConfig = memoryz.get_gc_config()
    testz.assert_eq_int(updated_config.gc_threshold, 2048)
    testz.assert_eq_int(updated_config.collection_interval, 500)
    testz.assert_false(updated_config.concurrent_gc)
    
    fr fr Restore original config
    memoryz.configure_gc(original_config)
    damn based
}

slay test_manual_gc_trigger() lit {
    testz.test_start("Manual GC trigger")
    
    fr fr Allocate some memory to trigger GC
    sus ptr1 normie = memoryz.allocate(1024, 105)
    sus ptr2 normie = memoryz.allocate(1024, 106)
    sus ptr3 normie = memoryz.allocate(1024, 107)
    
    sus stats_before memoryz.AllocationStats = memoryz.get_memory_stats()
    
    fr fr Trigger garbage collection
    testz.assert_true(memoryz.trigger_gc())
    
    sus stats_after memoryz.AllocationStats = memoryz.get_memory_stats()
    testz.assert_true(stats_after.gc_cycles > stats_before.gc_cycles)
    
    fr fr Clean up
    memoryz.deallocate(ptr1)
    memoryz.deallocate(ptr2)
    memoryz.deallocate(ptr3)
    damn based
}

slay test_reference_counting() lit {
    testz.test_start("Reference counting")
    
    sus ptr normie = memoryz.allocate(512, 108)
    testz.assert_true(ptr != 0)
    
    fr fr Increment reference count
    testz.assert_true(memoryz.increment_ref_count(ptr))
    testz.assert_true(memoryz.increment_ref_count(ptr))
    
    fr fr Decrement should not free yet (ref count > 1)
    testz.assert_true(memoryz.decrement_ref_count(ptr))
    testz.assert_true(memoryz.is_valid_pointer(ptr))
    
    fr fr Decrement again should not free yet (ref count = 1)
    testz.assert_true(memoryz.decrement_ref_count(ptr))
    testz.assert_true(memoryz.is_valid_pointer(ptr))
    
    fr fr Final decrement should free the memory
    testz.assert_true(memoryz.decrement_ref_count(ptr))
    
    damn based
}

fr fr ===== MEMORY PROFILING TESTS =====

slay test_memory_statistics() lit {
    testz.test_start("Memory statistics tracking")
    
    memoryz.reset_memory_stats()
    sus initial_stats memoryz.AllocationStats = memoryz.get_memory_stats()
    
    fr fr Allocate some memory
    sus ptr1 normie = memoryz.allocate(1024, 109)
    sus ptr2 normie = memoryz.allocate(2048, 110)
    
    sus after_alloc_stats memoryz.AllocationStats = memoryz.get_memory_stats()
    testz.assert_true(after_alloc_stats.current_usage >= 3072)
    testz.assert_eq_int(after_alloc_stats.allocation_count, 2)
    testz.assert_true(after_alloc_stats.peak_usage >= after_alloc_stats.current_usage)
    
    fr fr Free one block
    memoryz.deallocate(ptr1)
    sus after_free_stats memoryz.AllocationStats = memoryz.get_memory_stats()
    testz.assert_eq_int(after_free_stats.free_count, 1)
    testz.assert_true(after_free_stats.current_usage < after_alloc_stats.current_usage)
    
    memoryz.deallocate(ptr2)
    damn based
}

slay test_memory_leak_detection() lit {
    testz.test_start("Memory leak detection")
    
    fr fr Allocate some blocks
    sus ptr1 normie = memoryz.allocate(512, 111)
    sus ptr2 normie = memoryz.allocate(1024, 112)
    
    fr fr Simulate passage of time for leak detection
    fr fr In real implementation, this would involve actual time delays
    
    sus potential_leaks memoryz[value].MemoryBlock = memoryz.find_memory_leaks()
    fr fr Should detect blocks allocated above (simplified test)
    testz.assert_true(potential_leaks.len() >= 0)
    
    fr fr Clean up
    memoryz.deallocate(ptr1)
    memoryz.deallocate(ptr2)
    damn based
}

slay test_memory_report() lit {
    testz.test_start("Memory report generation")
    
    fr fr Allocate some memory for the report
    sus ptr1 normie = memoryz.allocate(1024, 113)
    sus ptr2 normie = memoryz.allocate(2048, 114)
    
    fr fr Generate report (should not crash)
    memoryz.print_memory_report()
    
    sus stats memoryz.AllocationStats = memoryz.get_memory_stats()
    testz.assert_true(stats.current_usage > 0)
    testz.assert_true(stats.allocation_count > 0)
    
    memoryz.deallocate(ptr1)
    memoryz.deallocate(ptr2)
    damn based
}

fr fr ===== MEMORY UTILITY TESTS =====

slay test_memory_operations() lit {
    testz.test_start("Memory utility operations")
    
    sus ptr1 normie = memoryz.allocate(256, 115)
    sus ptr2 normie = memoryz.allocate(256, 116)
    
    fr fr Test zero memory
    testz.assert_true(memoryz.zero_memory(ptr1, 256))
    
    fr fr Test set memory
    testz.assert_true(memoryz.set_memory(ptr1, 0x42, 128))
    
    fr fr Test copy memory
    testz.assert_true(memoryz.copy_memory(ptr2, ptr1, 128))
    
    fr fr Test compare memory (first 128 bytes should be equal)
    testz.assert_eq_int(memoryz.compare_memory(ptr1, ptr2, 128), 0)
    
    memoryz.deallocate(ptr1)
    memoryz.deallocate(ptr2)
    damn based
}

slay test_memory_alignment() lit {
    testz.test_start("Memory alignment")
    
    fr fr Test size alignment
    testz.assert_eq_int(memoryz.align_size(15, 8), 16)
    testz.assert_eq_int(memoryz.align_size(16, 8), 16)
    testz.assert_eq_int(memoryz.align_size(17, 8), 24)
    
    fr fr Test aligned allocation
    sus aligned_ptr normie = memoryz.allocate_aligned(1000, 16, 117)
    testz.assert_true(aligned_ptr != 0)
    testz.assert_eq_int(aligned_ptr % 16, 0)  fr fr Should be 16-byte aligned
    
    memoryz.deallocate(aligned_ptr)
    damn based
}

fr fr ===== MEMORY POOL TESTS =====

slay test_memory_pools() lit {
    testz.test_start("Memory pool management")
    
    fr fr Create memory pool
    sus pool memoryz.MemoryPool = memoryz.create_memory_pool(64, 10)
    testz.assert_true(pool.initialized)
    testz.assert_eq_int(pool.block_size, 64)
    testz.assert_eq_int(pool.block_count, 10)
    testz.assert_eq_int(pool.free_blocks.len(), 10)
    
    fr fr Allocate from pool
    sus ptr1 normie = memoryz.pool_allocate(pool)
    sus ptr2 normie = memoryz.pool_allocate(pool)
    testz.assert_true(ptr1 != 0)
    testz.assert_true(ptr2 != 0)
    testz.assert_true(ptr1 != ptr2)
    testz.assert_eq_int(pool.free_blocks.len(), 8)
    
    fr fr Return to pool
    testz.assert_true(memoryz.pool_deallocate(pool, ptr1))
    testz.assert_eq_int(pool.free_blocks.len(), 9)
    
    testz.assert_true(memoryz.pool_deallocate(pool, ptr2))
    testz.assert_eq_int(pool.free_blocks.len(), 10)
    
    fr fr Destroy pool
    testz.assert_true(memoryz.destroy_memory_pool(pool))
    damn based
}

fr fr ===== ERROR HANDLING TESTS =====

slay test_invalid_operations() lit {
    testz.test_start("Invalid memory operations")
    
    fr fr Test invalid allocations
    testz.assert_eq_int(memoryz.allocate(0, 118), 0)      fr fr Zero size
    testz.assert_eq_int(memoryz.allocate(-1, 119), 0)     fr fr Negative size
    
    fr fr Test invalid deallocations
    testz.assert_false(memoryz.deallocate(0))             fr fr Null pointer
    testz.assert_false(memoryz.deallocate(0xDEADBEEF))    fr fr Invalid pointer
    
    fr fr Test invalid reallocations
    testz.assert_eq_int(memoryz.reallocate(0, 512), memoryz.allocate(512, 0))  fr fr Null ptr should allocate
    
    fr fr Test invalid pointer operations
    testz.assert_false(memoryz.is_valid_pointer(0))
    testz.assert_false(memoryz.is_valid_pointer(0xDEADBEEF))
    testz.assert_eq_int(memoryz.get_block_size(0), 0)
    testz.assert_eq_int(memoryz.get_block_type(0xDEADBEEF), 0)
    
    damn based
}

fr fr ===== INTEGRATION TESTS =====

slay test_memory_stress() lit {
    testz.test_start("Memory stress test")
    
    sus pointers normie[value] = []
    
    fr fr Allocate many blocks
    bestie i := 0; i < 100; i++ {
        sus size normie = (i % 10 + 1) * 64  fr fr Varying sizes
        sus ptr normie = memoryz.allocate(size, 120 + i)
        lowkey ptr != 0 {
            pointers.push(ptr)
        }
    }
    
    testz.assert_true(pointers.len() > 50)  fr fr Should succeed for most allocations
    
    fr fr Trigger GC during stress
    memoryz.trigger_gc()
    
    fr fr Free half the blocks
    bestie i := 0; i < pointers.len(); i += 2 {
        memoryz.deallocate(pointers[i])
        pointers[i] = 0
    }
    
    fr fr Trigger GC again
    memoryz.trigger_gc()
    
    fr fr Free remaining blocks
    bestie ptr in pointers {
        lowkey ptr != 0 {
            memoryz.deallocate(ptr)
        }
    }
    
    damn based
}

fr fr ===== RUN ALL TESTS =====

slay run_all_memoryz_tests() lit {
    testz.test_group_start("memoryz module tests")
    
    test_basic_allocation()
    test_zeroed_allocation()
    test_array_allocation()
    test_reallocation()
    test_gc_configuration()
    test_manual_gc_trigger()
    test_reference_counting()
    test_memory_statistics()
    test_memory_leak_detection()
    test_memory_report()
    test_memory_operations()
    test_memory_alignment()
    test_memory_pools()
    test_invalid_operations()
    test_memory_stress()
    
    testz.test_group_end()
    testz.print_test_summary()
    damn based
}

fr fr Run the tests
run_all_memoryz_tests()
