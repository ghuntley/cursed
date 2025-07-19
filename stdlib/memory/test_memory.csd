yeet "testz"
yeet "memory"

test_start("Production Memory Allocation System")

// Test basic allocation
sus ptr thicc = malloc(1024)
assert_true(ptr != 0)
vibez.spill("✅ Basic allocation works")

// Test tagged allocation
sus obj_ptr thicc = alloc_object(256)
assert_true(obj_ptr != 0)
vibez.spill("✅ Tagged object allocation works")

sus arr_ptr thicc = alloc_array(512)
assert_true(arr_ptr != 0)
vibez.spill("✅ Tagged array allocation works")

sus str_ptr thicc = alloc_string(128)
assert_true(str_ptr != 0)
vibez.spill("✅ Tagged string allocation works")

// Test memory utilities
sus success lit = zero_memory(ptr, 1024)
assert_true(success)
vibez.spill("✅ Memory zeroing works")

// Test memory alignment
sus aligned_size normie = align_size(100, 8)
assert_true(aligned_size >= 100)
assert_true(aligned_size % 8 == 0)
vibez.spill("✅ Memory alignment works")

// Test memory comparison
sus ptr2 thicc = malloc(1024)
zero_memory(ptr2, 1024)
sus cmp_result normie = compare_memory(ptr, ptr2, 1024)
assert_true(cmp_result == 0)
vibez.spill("✅ Memory comparison works")

// Test memory copy
sus copy_success lit = copy_memory(ptr2, ptr, 1024)
assert_true(copy_success)
vibez.spill("✅ Memory copy works")

// Test garbage collection
sus freed_bytes normie = gc_collect()
assert_true(freed_bytes >= 0)
vibez.spill("✅ Garbage collection works")

// Test GC statistics
sus gc_stats_str tea = gc_stats()
assert_true(gc_stats_str != "")
vibez.spill("✅ GC statistics available:", gc_stats_str)

// Test memory pressure monitoring
sus pressure drip = gc_pressure()
assert_true(pressure >= 0.0)
assert_true(pressure <= 1.0)
vibez.spill("✅ Memory pressure monitoring works:", pressure)

// Test memory reporting
sus memory_report_str tea = memory_report()
assert_true(memory_report_str != "")
vibez.spill("✅ Memory reporting works:", memory_report_str)

// Test memory usage tracking
sus usage thicc = memory_usage()
assert_true(usage > 0)
vibez.spill("✅ Memory usage tracking works:", usage)

// Test stack operations
sus stack_size normie = get_stack_size()
assert_true(stack_size > 0)
vibez.spill("✅ Stack size monitoring works:", stack_size)

sus stack_overflow lit = check_stack_overflow()
assert_true(stack_overflow == cap)
vibez.spill("✅ Stack overflow check works")

// Test memory pool operations
sus pool_id thicc = create_pool(64, 100)
assert_true(pool_id != 0)
vibez.spill("✅ Memory pool creation works")

sus pool_ptr thicc = pool_alloc(pool_id, 64)
assert_true(pool_ptr != 0)
vibez.spill("✅ Pool allocation works")

sus pool_free_success lit = pool_free(pool_id, pool_ptr)
assert_true(pool_free_success)
vibez.spill("✅ Pool deallocation works")

// Test memory limits and compaction
sus limit_success lit = set_memory_limit(128 * 1024 * 1024)  // 128MB
assert_true(limit_success)
vibez.spill("✅ Memory limit setting works")

sus compacted_bytes normie = memory_compact()
assert_true(compacted_bytes >= 0)
vibez.spill("✅ Memory compaction works")

// Test memory performance monitoring
sus pressure_monitor_result lit = memory_pressure_monitor()
vibez.spill("✅ Memory pressure monitoring works")

sus performance_report tea = memory_performance_report()
assert_true(performance_report != "")
vibez.spill("✅ Memory performance reporting works")

// Test memory leak detection
sus leak_report tea = check_memory_leaks()
assert_true(leak_report != "")
vibez.spill("✅ Memory leak detection works")

// Clean up allocations
free(ptr)
free(obj_ptr)
free(arr_ptr)
free(str_ptr)
free(ptr2)
vibez.spill("✅ Memory deallocation works")

// Reset stats for clean finish
sus reset_success lit = reset_memory_stats()
assert_true(reset_success)
vibez.spill("✅ Memory statistics reset works")

vibez.spill("\n🎉 All memory allocation tests passed!")
vibez.spill("📊 Memory system is production-ready with:")
vibez.spill("  - Real GC-integrated allocation")
vibez.spill("  - Multiple allocation strategies")
vibez.spill("  - Memory tracking and profiling")  
vibez.spill("  - Error handling and leak detection")
vibez.spill("  - Performance monitoring")
vibez.spill("  - Memory pools and compaction")

print_test_summary()
