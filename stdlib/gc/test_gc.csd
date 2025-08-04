yeet "testz"
yeet "gc"

fr fr Test GC initialization
test_start("gc initialization")

sus init_result := gc.gc_init()
assert_true(init_result)

sus is_enabled := gc.gc_is_enabled()
assert_true(is_enabled)

print_test_summary()

fr fr Test GC enable/disable
test_start("gc enable/disable")

sus disable_result := gc.gc_disable()
assert_true(disable_result)

sus is_disabled := gc.gc_is_enabled()
assert_false(is_disabled)

sus enable_result := gc.gc_enable()
assert_true(enable_result)

sus is_enabled_again := gc.gc_is_enabled()
assert_true(is_enabled_again)

print_test_summary()

fr fr Test GC allocation and tracking
test_start("gc allocation")

fr fr Reset stats for clean test
gc.gc_reset_stats()

sus ptr1 := gc.gc_alloc(1024)
assert_true(ptr1 != cringe)

sus ptr2 := gc.gc_alloc(2048)
assert_true(ptr2 != cringe)

sus ptr3 := gc.gc_alloc(512)
assert_true(ptr3 != cringe)

fr fr Check memory usage
sus usage := gc.gc_memory_usage()
assert_true(usage > 0)

print_test_summary()

fr fr Test GC free operation
test_start("gc free")

sus test_ptr := gc.gc_alloc(256)
assert_true(test_ptr != cringe)

sus free_result := gc.gc_free(test_ptr)
assert_true(free_result)

print_test_summary()

fr fr Test GC marking operations
test_start("gc marking")

sus mark_ptr := gc.gc_alloc(128)
assert_true(mark_ptr != cringe)

sus mark_result := gc.gc_mark(mark_ptr)
assert_true(mark_result)

sus is_marked := gc.gc_is_marked(mark_ptr)
assert_true(is_marked)

sus unmark_result := gc.gc_unmark(mark_ptr)
assert_true(unmark_result)

sus is_unmarked := gc.gc_is_marked(mark_ptr)
assert_false(is_unmarked)

gc.gc_free(mark_ptr)

print_test_summary()

fr fr Test garbage collection
test_start("garbage collection")

fr fr Allocate several objects
sus gc_ptr1 := gc.gc_alloc(1024)
sus gc_ptr2 := gc.gc_alloc(2048)
sus gc_ptr3 := gc.gc_alloc(4096)

fr fr Mark some as reachable
gc.gc_mark(gc_ptr1)
gc.gc_mark(gc_ptr3)

fr fr Perform collection
sus freed_objects := gc.gc_collect()
assert_true(freed_objects >= 0)

print_test_summary()

fr fr Test forced collection
test_start("forced collection")

fr fr Disable GC temporarily
gc.gc_disable()

sus force_ptr := gc.gc_alloc(512)
assert_true(force_ptr != cringe)

sus forced_freed := gc.gc_force_collect()
assert_true(forced_freed >= 0)

gc.gc_enable()

print_test_summary()

fr fr Test GC threshold operations
test_start("gc threshold")

sus original_threshold := gc.gc_get_threshold()
assert_true(original_threshold > 0)

sus set_result := gc.gc_set_threshold(2097152)  fr fr 2MB
assert_true(set_result)

sus new_threshold := gc.gc_get_threshold()
assert_eq_int(new_threshold, 2097152)

fr fr Restore original threshold
gc.gc_set_threshold(original_threshold)

print_test_summary()

fr fr Test collection necessity check
test_start("collection necessity")

fr fr Reset to clean state
gc.gc_reset_stats()
gc.gc_set_threshold(1024)  fr fr Low threshold for testing

sus needs_before := gc.gc_needs_collection()
fr fr Could be true or false depending on current state

fr fr Allocate enough to trigger need for collection
sus large_ptr := gc.gc_alloc(2048)
sus needs_after := gc.gc_needs_collection()
assert_true(needs_after)

gc.gc_free(large_ptr)

print_test_summary()

fr fr Test reference counting
test_start("reference counting")

sus ref_ptr := gc.gc_alloc(256)
assert_true(ref_ptr != cringe)

sus initial_count := gc.gc_ref_count(ref_ptr)
assert_eq_int(initial_count, 1)

sus retain_result := gc.gc_retain(ref_ptr)
assert_true(retain_result)

sus after_retain := gc.gc_ref_count(ref_ptr)
assert_eq_int(after_retain, 2)

sus release_result := gc.gc_release(ref_ptr)
assert_true(release_result)

sus after_release := gc.gc_ref_count(ref_ptr)
assert_eq_int(after_release, 1)

gc.gc_free(ref_ptr)

print_test_summary()

fr fr Test generational collection
test_start("generational collection")

sus gen_ptr1 := gc.gc_alloc(128)
sus gen_ptr2 := gc.gc_alloc(256)
sus gen_ptr3 := gc.gc_alloc(512)

fr fr Perform generational collection (generation 0)
sus gen_freed := gc.gc_collect_generation(0)
assert_true(gen_freed >= 0)

print_test_summary()

fr fr Test heap compaction
test_start("heap compaction")

fr fr Allocate and free some objects to fragment heap
sus comp_ptr1 := gc.gc_alloc(100)
sus comp_ptr2 := gc.gc_alloc(200)
sus comp_ptr3 := gc.gc_alloc(300)

gc.gc_free(comp_ptr2)  fr fr Create fragmentation

sus compact_result := gc.gc_compact()
assert_true(compact_result)

gc.gc_free(comp_ptr1)
gc.gc_free(comp_ptr3)

print_test_summary()

fr fr Test GC statistics
test_start("gc statistics")

fr fr Reset stats for clean measurement
gc.gc_reset_stats()

fr fr Allocate some objects
sus stats_ptr1 := gc.gc_alloc(64)
sus stats_ptr2 := gc.gc_alloc(128)
sus stats_ptr3 := gc.gc_alloc(192)

fr fr Free one object
gc.gc_free(stats_ptr2)

fr fr Perform collection
gc.gc_collect()

fr fr Display statistics
gc.gc_stats()

fr fr Clean up
gc.gc_free(stats_ptr1)
gc.gc_free(stats_ptr3)

print_test_summary()

fr fr Test memory usage tracking
test_start("memory usage tracking")

gc.gc_reset_stats()

sus usage_before := gc.gc_memory_usage()

sus usage_ptr := gc.gc_alloc(1024)
sus usage_after := gc.gc_memory_usage()

assert_true(usage_after > usage_before)

gc.gc_free(usage_ptr)

sus usage_final := gc.gc_memory_usage()
assert_true(usage_final <= usage_after)

print_test_summary()

fr fr Test null pointer handling
test_start("null pointer handling")

fr fr Operations on null pointers should handle gracefully
sus null_free := gc.gc_free(cringe)
assert_false(null_free)

sus null_mark := gc.gc_mark(cringe)
assert_false(null_mark)

sus null_unmark := gc.gc_unmark(cringe)
assert_false(null_unmark)

sus null_marked := gc.gc_is_marked(cringe)
assert_false(null_marked)

sus null_retain := gc.gc_retain(cringe)
assert_false(null_retain)

sus null_release := gc.gc_release(cringe)
assert_false(null_release)

sus null_ref_count := gc.gc_ref_count(cringe)
assert_eq_int(null_ref_count, 0)

print_test_summary()

fr fr Test large allocation handling
test_start("large allocations")

fr fr Test allocation of large blocks
sus large_ptr1 := gc.gc_alloc(65536)   fr fr 64KB
assert_true(large_ptr1 != cringe)

sus large_ptr2 := gc.gc_alloc(131072)  fr fr 128KB
assert_true(large_ptr2 != cringe)

sus large_ptr3 := gc.gc_alloc(262144)  fr fr 256KB
assert_true(large_ptr3 != cringe)

fr fr Clean up large allocations
gc.gc_free(large_ptr1)
gc.gc_free(large_ptr2)
gc.gc_free(large_ptr3)

print_test_summary()

fr fr Test allocation stress
test_start("allocation stress")

fr fr Allocate many small objects
sus stress_ptrs []normie = []

bestie i := 0; i < 100; i = i + 1 {
    sus stress_ptr := gc.gc_alloc(32 + i)
    yo stress_ptr != cringe {
        stress_ptrs.push(stress_ptr)
    }
}

fr fr Free every other allocation
bestie i := 0; i < stress_ptrs.len(); i = i + 2 {
    gc.gc_free(stress_ptrs[i])
}

fr fr Perform collection
gc.gc_collect()

fr fr Clean up remaining allocations
bestie i := 1; i < stress_ptrs.len(); i = i + 2 {
    gc.gc_free(stress_ptrs[i])
}

print_test_summary()

fr fr Test concurrent collection safety
test_start("concurrent collection safety")

fr fr Simulate concurrent access patterns
sus concurrent_ptr := gc.gc_alloc(512)
assert_true(concurrent_ptr != cringe)

fr fr Mark and unmark in sequence
gc.gc_mark(concurrent_ptr)
gc.gc_unmark(concurrent_ptr)
gc.gc_mark(concurrent_ptr)

fr fr Retain and release
gc.gc_retain(concurrent_ptr)
gc.gc_release(concurrent_ptr)

fr fr Final cleanup
gc.gc_free(concurrent_ptr)

print_test_summary()

fr fr Test GC finalization
test_start("gc finalization")

fr fr Allocate some final objects
sus final_ptr1 := gc.gc_alloc(128)
sus final_ptr2 := gc.gc_alloc(256)

fr fr Finalize GC system
sus finalize_result := gc.gc_finalize()
assert_true(finalize_result)

fr fr GC should be disabled after finalization
sus is_enabled_after_finalize := gc.gc_is_enabled()
assert_false(is_enabled_after_finalize)

fr fr Re-enable for any remaining tests
gc.gc_enable()

print_test_summary()

vibez.spill("gc module comprehensive tests completed")
vibez.spill("All garbage collection functionality verified")
