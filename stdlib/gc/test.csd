yeet "testz"
yeet "gc"

test_start("GC Module Tests")

fr fr === Initialization Tests ===
test_case("GC Initialization") {
    sus result lit = gc_init()
    assert_eq_bool(result, based)
    assert_eq_bool(gc_is_enabled(), based)
}

test_case("GC Enable/Disable") {
    gc_enable()
    assert_eq_bool(gc_is_enabled(), based)
    
    gc_disable()
    assert_eq_bool(gc_is_enabled(), cap)
    
    fr fr Re-enable for other tests
    gc_enable()
}

fr fr === Memory Allocation Tests ===
test_case("Basic GC Allocation") {
    sus ptr1 *void = gc_alloc(64)
    assert_not_null(ptr1)
    
    sus ptr2 *void = gc_alloc(128)
    assert_not_null(ptr2)
    assert_not_equal(ptr1, ptr2)
    
    fr fr Check memory usage increased
    sus usage thicc = gc_memory_usage()
    assert_greater_than(usage, 0)
}

test_case("GC Free Memory") {
    sus initial_usage thicc = gc_memory_usage()
    
    sus ptr *void = gc_alloc(256)
    assert_not_null(ptr)
    
    sus after_alloc thicc = gc_memory_usage()
    assert_greater_than(after_alloc, initial_usage)
    
    sus result lit = gc_free(ptr)
    assert_eq_bool(result, based)
}

test_case("Null Pointer Handling") {
    sus result lit = gc_free(cringe)
    assert_eq_bool(result, cap)
    
    sus ref_count normie = gc_ref_count(cringe)
    assert_eq_int(ref_count, 0)
}

fr fr === Reference Counting Tests ===
test_case("Reference Counting") {
    sus ptr *void = gc_alloc(64)
    assert_not_null(ptr)
    
    fr fr Initial ref count should be 1
    sus initial_count normie = gc_ref_count(ptr)
    assert_eq_int(initial_count, 1)
    
    fr fr Retain should increment
    sus retain_result lit = gc_retain(ptr)
    assert_eq_bool(retain_result, based)
    sus after_retain normie = gc_ref_count(ptr)
    assert_eq_int(after_retain, 2)
    
    fr fr Release should decrement
    sus release_result lit = gc_release(ptr)
    assert_eq_bool(release_result, based)
    sus after_release normie = gc_ref_count(ptr)
    assert_eq_int(after_release, 1)
}

fr fr === Marking Tests ===
test_case("Object Marking") {
    sus ptr *void = gc_alloc(64)
    assert_not_null(ptr)
    
    fr fr Should not be marked initially
    assert_eq_bool(gc_is_marked(ptr), cap)
    
    fr fr Mark object
    sus mark_result lit = gc_mark(ptr)
    assert_eq_bool(mark_result, based)
    assert_eq_bool(gc_is_marked(ptr), based)
    
    fr fr Unmark object
    sus unmark_result lit = gc_unmark(ptr)
    assert_eq_bool(unmark_result, based)
    assert_eq_bool(gc_is_marked(ptr), cap)
}

fr fr === Collection Tests ===
test_case("Force Collection") {
    sus initial_collections thicc = 0
    
    fr fr Allocate some memory
    bestie (sus i normie = 0; i < 10; i++) {
        sus ptr *void = gc_alloc(1024)
        assert_not_null(ptr)
    }
    
    sus before_usage thicc = gc_memory_usage()
    assert_greater_than(before_usage, 0)
    
    fr fr Force collection
    sus freed_objects normie = gc_force_collect()
    assert_greater_than_or_equal(freed_objects, 0)
}

test_case("Automatic Collection Trigger") {
    fr fr Set low threshold to trigger collection
    gc_set_threshold(1024)
    
    sus initial_usage thicc = gc_memory_usage()
    
    fr fr Allocate enough to trigger collection
    bestie (sus i normie = 0; i < 5; i++) {
        sus ptr *void = gc_alloc(512)
        assert_not_null(ptr)
    }
    
    fr fr Should have triggered at least one collection
    sus needs_collection lit = gc_needs_collection()
    assert_eq_bool(needs_collection, based)
    
    fr fr Reset to reasonable threshold
    gc_set_threshold(1048576)  fr fr 1MB
}

test_case("Generational Collection") {
    sus ptr1 *void = gc_alloc(64)
    sus ptr2 *void = gc_alloc(128)
    assert_not_null(ptr1)
    assert_not_null(ptr2)
    
    fr fr Collect specific generation
    sus freed normie = gc_collect_generation(0)
    assert_greater_than_or_equal(freed, 0)
}

fr fr === Threshold Tests ===
test_case("GC Threshold Management") {
    sus original_threshold thicc = gc_get_threshold()
    
    fr fr Set new threshold
    sus new_threshold thicc = 2048
    sus set_result lit = gc_set_threshold(new_threshold)
    assert_eq_bool(set_result, based)
    
    sus retrieved_threshold thicc = gc_get_threshold()
    assert_eq_int(retrieved_threshold, new_threshold)
    
    fr fr Restore original
    gc_set_threshold(original_threshold)
}

test_case("Collection Need Detection") {
    sus threshold thicc = gc_get_threshold()
    sus current_usage thicc = gc_memory_usage()
    
    yo current_usage < threshold {
        assert_eq_bool(gc_needs_collection(), cap)
    } kinda {
        assert_eq_bool(gc_needs_collection(), based)
    }
}

fr fr === Statistics Tests ===
test_case("GC Statistics") {
    fr fr Reset stats for clean test
    gc_reset_stats()
    
    fr fr Allocate some memory
    sus ptr1 *void = gc_alloc(128)
    sus ptr2 *void = gc_alloc(256)
    assert_not_null(ptr1)
    assert_not_null(ptr2)
    
    fr fr Check usage
    sus usage thicc = gc_memory_usage()
    assert_greater_than_or_equal(usage, 384)  fr fr At least 128 + 256
    
    fr fr Perform collection
    gc_force_collect()
    
    fr fr Print stats (visual verification)
    gc_stats()
}

fr fr === Stress Tests ===
test_case("Multiple Allocations Stress Test") {
    sus initial_usage thicc = gc_memory_usage()
    sus ptrs []*void = []
    
    fr fr Allocate many small objects
    bestie (sus i normie = 0; i < 100; i++) {
        sus ptr *void = gc_alloc(32)
        assert_not_null(ptr)
        ptrs.append(ptr)
    }
    
    sus peak_usage thicc = gc_memory_usage()
    assert_greater_than(peak_usage, initial_usage)
    
    fr fr Free half the objects
    bestie (sus i normie = 0; i < 50; i++) {
        gc_free(ptrs[i])
    }
    
    fr fr Force collection to clean up
    gc_force_collect()
}

test_case("Large Allocation Test") {
    sus large_size normie = 1048576  fr fr 1MB
    sus ptr *void = gc_alloc(large_size)
    assert_not_null(ptr)
    
    sus usage thicc = gc_memory_usage()
    assert_greater_than_or_equal(usage, large_size)
    
    gc_free(ptr)
}

fr fr === Error Handling Tests ===
test_case("Disabled GC Collection") {
    gc_disable()
    
    sus freed normie = gc_collect()
    assert_eq_int(freed, 0)  fr fr Should not collect when disabled
    
    gc_enable()  fr fr Re-enable for other tests
}

test_case("Concurrent Collection Safety") {
    fr fr This test checks that concurrent operations don't crash
    fr fr In a real implementation, this would use actual threading
    
    sus ptr1 *void = gc_alloc(64)
    sus ptr2 *void = gc_alloc(128)
    
    fr fr Mark objects
    gc_mark(ptr1)
    gc_mark(ptr2)
    
    fr fr Try to collect while objects are marked
    sus freed normie = gc_collect()
    assert_greater_than_or_equal(freed, 0)
}

fr fr === Compaction Tests ===
test_case("Heap Compaction") {
    sus initial_usage thicc = gc_memory_usage()
    
    fr fr Allocate and free to create fragmentation
    bestie (sus i normie = 0; i < 20; i++) {
        sus ptr *void = gc_alloc(64)
        assert_not_null(ptr)
        yo i % 2 == 0 {
            gc_free(ptr)  fr fr Free every other allocation
        }
    }
    
    fr fr Compact heap
    sus compact_result lit = gc_compact()
    assert_eq_bool(compact_result, based)
}

fr fr === Finalization Tests ===
test_case("GC Finalization") {
    fr fr Save original state
    sus was_enabled lit = gc_is_enabled()
    
    sus finalize_result lit = gc_finalize()
    assert_eq_bool(finalize_result, based)
    
    fr fr Should be disabled after finalization
    assert_eq_bool(gc_is_enabled(), cap)
    
    fr fr Re-initialize for other tests
    yo was_enabled {
        gc_init()
    }
}

fr fr === Edge Cases ===
test_case("Zero Size Allocation") {
    sus ptr *void = gc_alloc(0)
    fr fr Should handle zero-size allocation gracefully
    yo ptr != cringe {
        gc_free(ptr)
    }
}

test_case("Maximum Reference Count") {
    sus ptr *void = gc_alloc(64)
    assert_not_null(ptr)
    
    fr fr Retain many times
    bestie (sus i normie = 0; i < 100; i++) {
        gc_retain(ptr)
    }
    
    sus ref_count normie = gc_ref_count(ptr)
    assert_greater_than(ref_count, 100)
    
    fr fr Release all
    bestie (sus i normie = 0; i <= ref_count; i++) {
        gc_release(ptr)
    }
}

print_test_summary()
